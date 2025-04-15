use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use deno_ast::{MediaType, ParseParams};
use deno_core::anyhow::anyhow;
use deno_core::error::ModuleLoaderError;
use deno_core::futures::FutureExt;
use deno_core::{
    resolve_import, ModuleCodeBytes, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, RequestedModuleType, ResolutionKind,
};

use deno_error::JsErrorBox;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use deno_core::anyhow::Result;

/// Our custom module loader.
pub struct ZinniaModuleLoader {
    module_root: Option<PathBuf>,
    // Cache mapping file_name to source_code
    code_cache: Arc<RwLock<HashMap<String, String>>>,
}

impl ZinniaModuleLoader {
    pub fn build(module_root: Option<PathBuf>) -> Result<Self> {
        let module_root = match module_root {
            None => None,
            // We must canonicalize the module root path too. It's best to do it once at startup.
            Some(r) => Some(r.canonicalize()?),
        };

        Ok(Self {
            module_root,
            code_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

pub fn get_module_root(main_js_module: &ModuleSpecifier) -> Result<PathBuf> {
    Ok(main_js_module
        .to_file_path()
        .map_err(|_| anyhow!("Invalid main module specifier: not a local path."))?
        .parent()
        .ok_or_else(|| anyhow!("Invalid main module specifier: it has no parent directory!"))?
        // Resolve any symlinks inside the path to prevent modules from escaping our sandbox
        .canonicalize()?)
}

impl ModuleLoader for ZinniaModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: ResolutionKind,
    ) -> Result<ModuleSpecifier, ModuleLoaderError> {
        if specifier == "zinnia:test" {
            return Ok(ModuleSpecifier::parse("ext:zinnia_runtime/test.js").unwrap());
        } else if specifier == "zinnia:assert" {
            return Ok(
                ModuleSpecifier::parse("ext:zinnia_runtime/vendored/asserts.bundle.js").unwrap(),
            );
        }

        let resolved = resolve_import(specifier, referrer)?;
        Ok(resolved)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        requested_module_type: RequestedModuleType,
    ) -> ModuleLoadResponse {
        let module_specifier = module_specifier.clone();
        let module_root = self.module_root.clone();
        let maybe_referrer = maybe_referrer.cloned();
        let code_cache = self.code_cache.clone();
        let module_load = async move {
            let spec_str = module_specifier.as_str();

            let details = || {
                let mut msg = format!("\nModule URL: {spec_str}");
                if let Some(referrer) = &maybe_referrer {
                    msg.push_str("\nImported from: ");
                    msg.push_str(referrer.as_str());
                }
                msg
            };

            if module_specifier.scheme() != "file" {
                let msg = format!(
                    "Unsupported scheme: {}. Zinnia can import local modules only.{}",
                    module_specifier.scheme(),
                    details()
                );
                return Err(ModuleLoaderError::from(JsErrorBox::generic(msg)));
            }

            let module_path = module_specifier.to_file_path().map_err(|_| {
                let msg = format!(
                    "Module specifier cannot be converted to a filepath.{}",
                    details()
                );
                ModuleLoaderError::from(JsErrorBox::generic(msg))
            })?;

            let sandboxed_path: PathBuf;

            // Check that the module path is inside the module root directory
            if let Some(canonical_root) = &module_root {
                // Resolve any symlinks inside the path to prevent modules from escaping our sandbox
                let canonical_module = module_path.canonicalize().map_err(|err| {
                    let msg = format!(
                        "Cannot canonicalize module path: {err}.\nModule file path: {}{}",
                        module_path.display(),
                        details()
                    );
                    ModuleLoaderError::from(JsErrorBox::generic(msg))
                })?;

                let relative_path =
                    canonical_module.strip_prefix(canonical_root).map_err(|_| {
                        let msg = format!(
                            "Cannot import files outside of the module root directory.\n\
                         Root directory (canonical): {}\n\
                         Module file path (canonical): {}\
                         {}",
                            canonical_root.display(),
                            canonical_module.display(),
                            details()
                        );
                        ModuleLoaderError::from(JsErrorBox::generic(msg))
                    })?;

                let virtual_root = if cfg!(target_os = "windows") {
                    r"C:\ZINNIA"
                } else {
                    r"/ZINNIA"
                };
                sandboxed_path = Path::new(virtual_root).join(relative_path).to_owned();
            } else {
                sandboxed_path = module_path.to_owned();
            };

            // Based on https://github.com/denoland/roll-your-own-javascript-runtime
            let media_type = MediaType::from_path(&module_path);
            log::debug!("Loading module: {}", module_path.display());
            log::debug!("Media type: {:?}", media_type);
            let (module_type, should_transpile) = match MediaType::from_path(&module_path) {
                MediaType::JavaScript => (ModuleType::JavaScript, false),
                MediaType::TypeScript => (ModuleType::JavaScript, true),
                MediaType::Json => (ModuleType::Json, false),
                MediaType::Wasm => (ModuleType::Wasm, false),
                _ => {
                    return Err(ModuleLoaderError::Unsupported {
                        specifier: module_specifier.into(),
                        maybe_referrer: maybe_referrer.map(|r| r.into()),
                    })
                }
            };
            log::debug!(
                "Module type: {} Should transpile: {}",
                module_type,
                should_transpile
            );

            // If we loaded a JSON file, but the "requested_module_type" (that is computed from
            // import attributes) is not JSON we need to fail.
            if module_type == ModuleType::Json && requested_module_type != RequestedModuleType::Json
            {
                return Err(JsErrorBox::generic("Attempted to load JSON module without specifying \"type\": \"json\" attribute in the import statement.").into());
            }

            if module_type == ModuleType::Wasm {
                let code = read_file(&module_path).await?;
                let module = ModuleSource::new(
                    module_type,
                    ModuleSourceCode::Bytes(ModuleCodeBytes::Boxed(code.into_boxed_slice())),
                    &module_specifier,
                    None,
                );
                return Ok(module);
            }

            let code = read_file_to_string(&module_path).await?;

            let code = if should_transpile {
                let parsed = deno_ast::parse_module(ParseParams {
                    specifier: module_specifier.clone(),
                    text: code.into(),
                    media_type,
                    capture_tokens: false,
                    scope_analysis: false,
                    maybe_syntax: None,
                })
                .map_err(JsErrorBox::from_err)?;
                parsed
                    .transpile(
                        &deno_ast::TranspileOptions {
                            imports_not_used_as_values: deno_ast::ImportsNotUsedAsValues::Error,
                            verbatim_module_syntax: true,
                            ..Default::default()
                        },
                        &Default::default(),
                        &Default::default(),
                    )
                    .map_err(JsErrorBox::from_err)?
                    .into_source()
                    .text
            } else {
                code
            };

            code_cache
                .write()
                .map_err(|_| {
                    JsErrorBox::generic("Unexpected internal error: code_cache lock was poisoned")
                })?
                .insert(spec_str.to_string(), code.clone());

            let sandboxed_module_specifier = ModuleSpecifier::from_file_path(&sandboxed_path)
                .map_err(|_| {
                    let msg = format!(
                        "Internal error: cannot convert relative path to module specifier: {}\n{}",
                        sandboxed_path.display(),
                        details()
                    );
                    ModuleLoaderError::from(JsErrorBox::generic(msg))
                })?;

            let module = ModuleSource::new(
                module_type,
                ModuleSourceCode::String(code.into()),
                &sandboxed_module_specifier,
                None,
            );

            Ok(module)
        };

        ModuleLoadResponse::Async(module_load.boxed_local())
    }

    fn get_source_mapped_source_line(&self, file_name: &str, line_number: usize) -> Option<String> {
        log::debug!("get_source_mapped_source_line {file_name}:{line_number}");
        let code_cache = self.code_cache.read().ok()?;
        let code = code_cache.get(file_name)?;

        // Based on Deno cli/module_loader.rs
        // https://github.com/denoland/deno/blob/32b9cc91d8c343bdec2ddcf3cedb27b5efc2f5e4/cli/module_loader.rs#L1195-L1218

        // Do NOT use .lines(): it skips the terminating empty line.
        // (due to internally using_terminator() instead of .split())
        let lines: Vec<&str> = code.split('\n').collect();
        if line_number >= lines.len() {
            Some(format!(
          "{} Couldn't format source line: Line {} is out of bounds (source may have changed at runtime)",
          crate::colors::yellow("Warning"), line_number + 1,
        ))
        } else {
            Some(lines[line_number].to_string())
        }
    }
}

async fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>, ModuleLoaderError> {
    let mut f = File::open(&path).await?;

    // read the whole file
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    Ok(buffer)
}

async fn read_file_to_string(path: impl AsRef<Path>) -> Result<String, ModuleLoaderError> {
    let buffer = read_file(path).await?;
    Ok(String::from_utf8_lossy(&buffer).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use deno_core::{anyhow::Context, RequestedModuleType};
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn allows_import_of_files_inside_sandbox() {
        let mut imported_file = get_js_dir();
        imported_file.push("99_main.js");

        let loader = ZinniaModuleLoader::build(Some(get_js_dir())).unwrap();
        let response = loader.load(
            &ModuleSpecifier::from_file_path(&imported_file).unwrap(),
            None,
            false,
            RequestedModuleType::None,
        );
        let result = get_load_result(response)
            .await
            .with_context(|| format!("cannot import {}", imported_file.display()))
            .unwrap();

        assert_eq!(result.module_type, ModuleType::JavaScript);
    }

    #[tokio::test]
    async fn rejects_import_of_files_outside_sandbox() {
        // project_root is `runtime/tests/js`
        let mut project_root = get_js_dir().parent().unwrap().to_path_buf();
        project_root.push("tests");
        project_root.push("js");

        // we are importing file `runtime/js/99_main.js` - it's outside for project_root
        let mut imported_file = get_js_dir();
        imported_file.push("99_main.js");

        let loader = ZinniaModuleLoader::build(Some(project_root)).unwrap();
        let response = loader.load(
            &ModuleSpecifier::from_file_path(&imported_file).unwrap(),
            None,
            false,
            RequestedModuleType::None,
        );
        let result = get_load_result(response).await;

        match result {
            Ok(_) => {
                assert!(
                    result.is_err(),
                    "Expected import from '{}' to fail, it succeeded instead.",
                    imported_file.display()
                );
            }
            Err(err) => {
                let msg = format!("{err}");
                assert!(
                    msg.contains("Cannot import files outside of the module root directory"),
                    "Expected import to fail with the sandboxing error, it failed with a different error instead:\n{}",
                    msg,
                );
            }
        }
    }

    fn get_js_dir() -> PathBuf {
        let mut base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        base_dir.push("js");
        base_dir
    }

    async fn get_load_result(
        load_response: ModuleLoadResponse,
    ) -> Result<ModuleSource, ModuleLoaderError> {
        match load_response {
            ModuleLoadResponse::Sync(result) => result,
            ModuleLoadResponse::Async(fut) => fut.await,
        }
    }
}
