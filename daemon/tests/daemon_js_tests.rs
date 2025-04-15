use std::path::PathBuf;
use std::process::{Command, Stdio};
use tempfile::tempdir;
use zinnia_runtime::AnyError;

macro_rules! js_tests(
    ( $name:ident ) => {
    #[tokio::test]
    async fn $name() -> Result<(), AnyError> {
        run_js_test_file(&format!("{}.js", stringify!($name))).await?;
        Ok(())
    }
    };
);

js_tests!(source_code_paths);

async fn run_js_test_file(name: &str) -> Result<(), AnyError> {
    let temp_root = tempdir().expect("cannot create temporary directory");
    let cache_root = temp_root.path().join("cache");
    let state_root = temp_root.path().join("state");

    let mut mod_js = get_base_dir();
    mod_js.push(name);
    assert!(
        mod_js.is_file(),
        "test JS file not found: {}",
        mod_js.display()
    );

    let bin = assert_cmd::cargo::cargo_bin("zinniad");
    assert!(bin.is_file(), "zinniad not found: {}", bin.display());

    // Create a command to start zinniad
    let mut cmd = Command::new(bin);
    cmd.env("NO_COLOR", "1")
        .env("FIL_WALLET_ADDRESS", "f1test")
        .env("STATION_ID", "a".repeat(88))
        .env("CACHE_ROOT", cache_root.display().to_string())
        .env("STATE_ROOT", state_root.display().to_string())
        .args([&mod_js.as_os_str()]);

    // Hide activity logs in JSON format
    cmd.stdout(Stdio::null());

    let exit_status = cmd.status()?;
    assert!(
        exit_status.success(),
        "Expected zinniad to exit with success. Actual exit status: ${exit_status}"
    );
    Ok(())
}

fn get_base_dir() -> PathBuf {
    let mut base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    base_dir.push("tests");
    base_dir.push("js");
    base_dir
}
