use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use deno_core::{located_script_name, serde_json, JsRuntime, ModuleSpecifier, RuntimeOptions};

use deno_web::BlobStore;

use {once_cell::sync::Lazy, regex::Regex};

use crate::module_loader::ZinniaModuleLoader;
use crate::Reporter;

use crate::ext::ZinniaPermissions;

pub type AnyError = deno_core::anyhow::Error;
use deno_core::anyhow::{anyhow, Result};

/// Common bootstrap options for MainWorker & WebWorker
#[derive(Clone)]
pub struct BootstrapOptions {
    /// The user agent version string to use for Fetch API requests
    pub agent_version: String,

    /// Seed value for initializing the random number generator
    pub rng_seed: Option<u64>,

    /// Module root if you want to sandbox `import` of ES modules
    pub module_root: Option<PathBuf>,

    /// Filecoin wallet address - typically the built-in wallet in Filecoin Station
    pub wallet_address: String,

    /// Station ID - the unique identifier of the Filecoin Station
    pub station_id: String,

    /// Report activities
    pub reporter: Rc<dyn Reporter>,

    /// Lassie daemon to use as the IPFS retrieval client. We must use Arc here to allow sharing of
    /// the singleton Lassie instance between multiple threads spawned by Rust's test runner.
    pub lassie_daemon: Arc<lassie::Daemon>,

    /// Zinnia version reported by `Zinnia.versions.zinnia` API.
    /// Embedders can customize this value.
    pub zinnia_version: &'static str,
}

impl BootstrapOptions {
    pub fn new(
        agent_version: String,
        reporter: Rc<dyn Reporter>,
        lassie_daemon: Arc<lassie::Daemon>,
        module_root: Option<PathBuf>,
    ) -> Self {
        Self {
            agent_version,
            rng_seed: None,
            module_root,
            wallet_address: String::from("0x000000000000000000000000000000000000dEaD"),
            // Station ID must look like a public key - 88 hexadecimal characters.
            // Let's use all-zeroes value to make it easy to distinguish data reported
            // from non-production systems (dev, CI).
            station_id: "0".repeat(88),
            reporter,
            lassie_daemon,
            zinnia_version: env!("CARGO_PKG_VERSION"),
        }
    }

    pub fn as_json(&self) -> String {
        let payload = serde_json::json!({
          "walletAddress": self.wallet_address,
          "stationId": self.station_id,
          "lassieUrl": format!("http://127.0.0.1:{}/", self.lassie_daemon.port()),
          "lassieAuth": match self.lassie_daemon.access_token() {
            Some(token) => serde_json::Value::String(format!("Bearer {token}")),
            None => serde_json::Value::Null,
          },
          "zinniaVersion": self.zinnia_version,
          "v8Version": deno_core::v8::VERSION_STRING,
        });
        serde_json::to_string_pretty(&payload).unwrap()
    }
}

pub async fn run_js_module(
    module_specifier: &ModuleSpecifier,
    bootstrap_options: &BootstrapOptions,
) -> Result<(), AnyError> {
    if !validate_station_id(&bootstrap_options.station_id) {
        return Err(anyhow!("Invalid station_id format"));
    }

    let blob_store = Arc::new(BlobStore::default());
    let reporter = Rc::clone(&bootstrap_options.reporter);

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![
            // Web Platform APIs implemented by Deno plus their dependencies
            deno_telemetry::deno_telemetry::init_ops_and_esm(),
            deno_console::deno_console::init_ops_and_esm(),
            deno_webidl::deno_webidl::init_ops_and_esm(),
            deno_url::deno_url::init_ops_and_esm(),
            deno_web::deno_web::init_ops_and_esm::<ZinniaPermissions>(
                blob_store,
                Some(module_specifier.clone()),
            ),
            deno_fetch::deno_fetch::init_ops_and_esm::<ZinniaPermissions>(deno_fetch::Options {
                user_agent: bootstrap_options.agent_version.clone(),
                ..Default::default()
            }),
            deno_crypto::deno_crypto::init_ops_and_esm(bootstrap_options.rng_seed),
            deno_net::deno_net::init_ops_and_esm::<ZinniaPermissions>(None, None),
            deno_tls::deno_tls::init_ops_and_esm(),
            // Zinnia-specific APIs
            crate::ext::zinnia_runtime::init_ops_and_esm(reporter),
        ],
        extension_transpiler: Some(Rc::new(|specifier, source| {
            crate::vendored::transpile::maybe_transpile_source(specifier, source)
        })),
        inspector: false,
        module_loader: Some(Rc::new(ZinniaModuleLoader::build(
            bootstrap_options.module_root.clone(),
        )?)),
        ..Default::default()
    });

    let script = format!("bootstrap.mainRuntime({})", bootstrap_options.as_json());
    runtime.execute_script(located_script_name!(), script)?;

    // Load and run the module
    let main_module_id = runtime.load_main_es_module(module_specifier).await?;
    let res = runtime.mod_evaluate(main_module_id);
    runtime.run_event_loop(Default::default()).await?;
    res.await?;

    Ok(())
}

use deno_crypto::rand::{self, distributions::Alphanumeric, Rng};

const ONE_DAY: Duration = Duration::from_secs(24 * 3600);

/// A baseline configuration for Lassie shared by `zinnia` and `zinniad` CLIs.
pub fn lassie_config() -> lassie::DaemonConfig {
    let access_token = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(24)
        .map(char::from)
        .collect();

    lassie::DaemonConfig {
        port: 0,
        access_token: Some(access_token),
        provider_timeout: Some(ONE_DAY),
        global_timeout: Some(ONE_DAY),
        ..Default::default()
    }
}

fn validate_station_id(station_id: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9a-fA-F]{88}$").unwrap());
    RE.is_match(station_id)
}

pub fn exit(code: i32) -> ! {
    deno_telemetry::flush();
    #[allow(clippy::disallowed_methods)]
    std::process::exit(code);
}
