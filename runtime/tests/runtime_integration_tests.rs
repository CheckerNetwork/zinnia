// Runs tests defined in files inside the `js` directory
// The test files are valid Zinnia modules, so you can run them without Rust:
//   ./target/debug/zinnia run runtime/tests/js/timers_tests.js
// Most of the tests should pass on Deno too!
//   deno run runtime/tests/js/timers_tests.js
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::{anyhow, Context};
use deno_core::ModuleSpecifier;
use zinnia_runtime::fmt_errors::format_js_error;
use zinnia_runtime::{any_and_jserrorbox_downcast_ref, CoreError, RecordingReporter};
use zinnia_runtime::{anyhow, deno_core, run_js_module, AnyError, BootstrapOptions};

use pretty_assertions::assert_eq;

mod helpers;

macro_rules! js_tests(
    ( $name:ident ) => {
    #[tokio::test]
    async fn $name() -> Result<(), AnyError> {
        run_js_test_file(&format!("{}.js", stringify!($name))).await?;
        Ok(())
    }
    };

    ( $name:ident check_activity) => {
    #[tokio::test]
    async fn $name() -> Result<(), AnyError> {
        let activities = run_js_test_file(&format!("{}.js", stringify!($name))).await?;

        let actual_output = format_recorded_activities(&activities);
        let expected_output = load_activity_log(&format!("{}.activity.txt", stringify!($name)));
        assert_eq!(actual_output, expected_output);
        Ok(())
    }
    };
);

macro_rules! test_runner_tests(
    ( $name:ident ) => {
    #[tokio::test]
    async fn $name() -> Result<(), AnyError> {
        let activities = run_js_test_file(&format!("test_runner_tests/{}.js", stringify!($name))).await?;

        let actual_output = format_test_activities(&activities);
        let expected_output = load_activity_log(&format!("test_runner_tests/{}.activity.txt", stringify!($name)));

        assert_eq!(actual_output, expected_output);
        Ok(())
    }
    };

    ( $name:ident expect_failure ) => {
        #[tokio::test]
        async fn $name() -> Result<(), AnyError> {
            let result = run_js_test_file(&format!("test_runner_tests/{}.js", stringify!($name))).await;

            let activities = match result {
                Ok(_) => return Err(anyhow!("The test runner was expected to throw an error. Success was reported instead.")),
                Err(failure) => {
                    assert_test_runner_failure(failure.error);
                    failure.activities
                },
            };

            let actual_output = format_test_activities(&activities);
            let expected_output = load_activity_log(&format!("test_runner_tests/{}.activity.txt", stringify!($name)));

            assert_eq!(actual_output, expected_output);
            Ok(())
        }
        };
);

js_tests!(globals_tests);
js_tests!(versions_tests);
js_tests!(timers_tests);
js_tests!(webapis_tests);
js_tests!(webcrypto_tests);
js_tests!(station_apis_tests);
js_tests!(station_reporting_tests check_activity);
js_tests!(module_loader_tests);
js_tests!(fetch_tests);
js_tests!(ipfs_retrieval_tests);

test_runner_tests!(passing_tests);
test_runner_tests!(failing_tests expect_failure);

#[tokio::test]
async fn typescript_stack_trace_test() -> Result<(), AnyError> {
    let result = run_js_test_file("typescript_fixtures/typescript_stack_trace.ts").await;
    let error = match result {
        Ok(_) => {
            return Err(anyhow!(
                "The script was expected to throw an error. Success was reported instead."
            ))
        }
        Err(err) => err.error,
    };

    if let Some(CoreError::Js(e)) = any_and_jserrorbox_downcast_ref::<CoreError>(&error) {
        let actual_error = format_js_error(e);
        // Strip ANSI codes (colors, styles)
        let actual_error = console_static_text::ansi::strip_ansi_codes(&actual_error);
        // Replace current working directory in stack trace file paths with a fixed placeholder
        let cwd_url = ModuleSpecifier::from_file_path(std::env::current_dir().unwrap()).unwrap();
        let actual_error = actual_error.replace(cwd_url.as_str(), "file:///project-root");
        // Normalize line endings to Unix style (LF only)
        let actual_error = actual_error.replace("\r\n", "\n");

        let expected_error = r#"
Uncaught (in promise) Error
const error: Error = new Error(); throw error;
                     ^
    at file:///project-root/tests/js/typescript_fixtures/typescript_stack_trace.ts:14:22
"#;
        assert_eq!(actual_error.trim(), expected_error.trim());
    } else {
        panic!("The script threw unexpected error: {}", error);
    }
    Ok(())
}

#[tokio::test]
async fn source_code_paths_when_no_module_root() -> Result<(), AnyError> {
    let activities = run_js_test_file_with_module_root("print_source_code_paths.js", None).await?;

    let base_dir = get_base_dir();
    let dirname = base_dir.to_str().unwrap().to_string();
    let filename = Path::join(&base_dir, "print_source_code_paths.js").to_owned();
    let filename = filename.to_str().unwrap().to_string();
    let module_url = ModuleSpecifier::from_file_path(&filename).unwrap();
    let imported_module_filename = Path::join(&base_dir, "module_fixtures")
        .join("import_meta_filename.js")
        .to_owned();
    let imported_module_filename = imported_module_filename.to_str().unwrap().to_string();

    assert_eq!(
        [
            format!("imported module filename: {imported_module_filename}"),
            format!("import.meta.filename: {filename}"),
            format!("import.meta.dirname: {dirname}"),
            format!("error stack: at {module_url}:6:29"),
        ]
        .map(|msg| { format!("console.info: {msg}\n") }),
        activities.as_slice(),
    );
    Ok(())
}

#[tokio::test]
async fn source_code_paths_when_inside_module_root() -> Result<(), AnyError> {
    let module_root = Some(PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let activities =
        run_js_test_file_with_module_root("print_source_code_paths.js", module_root).await?;

    let base_dir = get_base_dir();
    let dirname = base_dir.to_str().unwrap().to_string();
    let filename = Path::join(&base_dir, "print_source_code_paths.js").to_owned();
    let filename = filename.to_str().unwrap().to_string();
    let module_url = ModuleSpecifier::from_file_path(&filename).unwrap();
    let imported_module_filename = Path::join(&base_dir, "module_fixtures")
        .join("import_meta_filename.js")
        .to_owned();
    let imported_module_filename = imported_module_filename.to_str().unwrap().to_string();

    // TODO: We want these paths to be mapped to a virtual root `/ZINNIA` or `C:\ZINNIA`
    // let expected = if cfg!(target_os = "windows") {
    //     [
    //         r"imported module filename: C:\ZINNIA\tests\js\js\module_fixtures\import_meta_filename.js",
    //         r"import.meta.filename: C:\ZINNIA\tests\js\print_source_code_paths.js",
    //         r"import.meta.dirname: C:\ZINNIA\tests\js",
    //         "error stack: at file:///C:/ZINNIA/tests/js/print_source_code_paths.js:3:29",
    //     ]
    // } else {
    //     [
    //         "imported module filename: /ZINNIA/tests/js/module_fixtures/import_meta_filename.js",
    //         "import.meta.filename: /ZINNIA/tests/js/print_source_code_paths.js",
    //         "import.meta.dirname: /ZINNIA/tests/js",
    //         "error stack: at file:///ZINNIA/tests/js/print_source_code_paths.js:6:29",
    //     ]
    // };
    let expected = [
        format!("imported module filename: {imported_module_filename}"),
        format!("import.meta.filename: {filename}"),
        format!("import.meta.dirname: {dirname}"),
        format!("error stack: at {module_url}:6:29"),
    ];

    assert_eq!(
        expected.map(|msg| { format!("console.info: {msg}\n") }),
        activities.as_slice(),
    );
    Ok(())
}

// Run all tests in a single JS file
async fn run_js_test_file(name: &str) -> Result<Vec<String>, RunFailure> {
    run_js_test_file_with_module_root(name, None).await
}

#[derive(Debug)]
struct RunFailure {
    error: AnyError,
    activities: Vec<String>,
}

impl RunFailure {
    fn new(error: AnyError, activities: Vec<String>) -> Self {
        Self { error, activities }
    }
}

impl From<AnyError> for RunFailure {
    fn from(err: AnyError) -> Self {
        RunFailure::new(err, vec![])
    }
}

impl From<RunFailure> for AnyError {
    fn from(err: RunFailure) -> Self {
        err.error
    }
}

async fn run_js_test_file_with_module_root(
    name: &str,
    module_root: Option<PathBuf>,
) -> Result<Vec<String>, RunFailure> {
    let _ = env_logger::builder().is_test(true).try_init();

    let mut full_path = get_base_dir();
    full_path.push(name);

    let main_module = deno_core::resolve_path(
        &full_path.to_string_lossy(),
        &std::env::current_dir().context("unable to get current working directory")?,
    )
    .map_err(|err| AnyError::from(err))?;
    let reporter = Rc::new(RecordingReporter::new());
    let config = BootstrapOptions::new(
        format!("zinnia_runtime_tests/{}", env!("CARGO_PKG_VERSION")),
        reporter.clone(),
        helpers::lassie_daemon(),
        module_root,
    );
    let run_result = run_js_module(&main_module, &config).await;
    let activities = reporter.events.take();

    match run_result {
        Ok(()) => Ok(activities),
        Err(err) => Err(RunFailure::new(err, activities)),
    }
}

fn get_base_dir() -> PathBuf {
    let mut base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base_dir.push("tests");
    base_dir.push("js");
    base_dir
}

fn get_activity_log_path(log_file: &str) -> PathBuf {
    let mut activity_path = get_base_dir();
    activity_path.push(log_file);
    activity_path
}

fn load_activity_log(log_file: &str) -> String {
    let file_path = get_activity_log_path(log_file);
    std::fs::read_to_string(&file_path)
        .unwrap_or_else(|err| panic!("cannot read {}: {}", file_path.display(), err))
        // normalize line endings to Unix style (LF only)
        .replace("\r\n", "\n")
}

fn format_recorded_activities(events: &[String]) -> String {
    events
        .iter()
        .map(|e| format!("{}\n", e.trim_end()))
        .collect::<Vec<String>>()
        .join("")
}

fn format_test_activities(events: &[String]) -> String {
    // Find all durations (e.g. `0ms` or `241ms`)
    let duration_regex = regex::Regex::new(r"\d+ms").unwrap();

    // Find trailing whitespace on all lines. Note that a chunk can be multi-line!
    let eol_regex = regex::Regex::new(r" *\r?\n").unwrap();

    let cwd_url = ModuleSpecifier::from_file_path(std::env::current_dir().unwrap()).unwrap();

    events
        .iter()
        .map(|chunk| {
            // Strip ANSI codes (colors, styles)
            let chunk = console_static_text::ansi::strip_ansi_codes(chunk);

            // Remove `console.info: ` added by RecordingReporter.
            // Don't remove other `console` level prefixes, so that we can detect them.
            let chunk = match chunk.strip_prefix("console.info: ") {
                Some(remainder) => remainder,
                None => &chunk,
            };

            // Replace current working directory in stack trace file paths with a fixed placeholder
            let chunk = chunk.replace(cwd_url.as_str(), "file:///project-root");

            // Normalize durations
            let chunk = duration_regex.replace_all(&chunk, "XXms");

            // Remove trailing whitespace before all EOLs
            let chunk = eol_regex.replace_all(&chunk, "\n");

            // Format the line, adding back EOL after trimming whitespace at the end
            format!("{}\n", chunk.trim_end())
        })
        .collect::<Vec<String>>()
        .join("")
}

fn assert_test_runner_failure(error: AnyError) {
    if let Some(CoreError::Js(e)) = any_and_jserrorbox_downcast_ref::<CoreError>(&error) {
        assert_eq!(
            e.name,
            Some("[some tests failed]\u{001b}[2K\x0D".to_string()),
            "error.name"
        );
        assert_eq!(e.message, None, "error.message");
        assert_eq!(e.stack, None, "error.stack");
    } else {
        panic!("The test runner threw unexpected error: {}", error);
    }
}
