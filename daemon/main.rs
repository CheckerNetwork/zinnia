mod args;
mod state;
mod station_reporter;

use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use args::CliArgs;
use clap::Parser;

use zinnia_runtime::anyhow::{anyhow, Context, Error, Result};
use zinnia_runtime::{get_module_root, lassie, resolve_path, run_js_module, BootstrapOptions};

use crate::station_reporter::{log_info_activity, StationReporter};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    setup_logger();
    let cli_args = CliArgs::parse_from(std::env::args());

    match run(cli_args).await {
        Ok(_) => (),
        Err(err) => exit_with_error(err),
    }
}

async fn run(config: CliArgs) -> Result<()> {
    log::info!("Starting zinniad with config {config:?}");

    if config.files.is_empty() {
        return Err(anyhow!("You must provide at least one module to run."));
    }
    if config.files.len() > 1 {
        return Err(anyhow!(
            "We do not yet support running more than one module."
        ));
    }

    let state_file = PathBuf::from(config.state_root).join("state.json");
    log::debug!("Using state file: {}", state_file.display());
    let lassie_temp_dir = PathBuf::from(config.cache_root).join("lassie");

    setup_lassie_tempdir(&lassie_temp_dir)?;

    let lassie_config = lassie::DaemonConfig {
        temp_dir: Some(lassie_temp_dir),
        port: 0,
    };
    let lassie_daemon = Arc::new(
        lassie::Daemon::start(lassie_config)
            .context("cannot initialize the IPFS retrieval client Lassie")?,
    );

    log_info_activity("Module Runtime started.");

    let file = &config.files[0];

    // TODO: configurable module name and version
    // https://github.com/filecoin-station/zinnia/issues/147
    let module_name = file.trim_end_matches(".js");

    let main_module = resolve_path(
        file,
        &std::env::current_dir().context("unable to get current working directory")?,
    )?;
    let module_root = get_module_root(&main_module)?;

    let runtime_config = BootstrapOptions {
        zinnia_version: env!("CARGO_PKG_VERSION"),
        agent_version: format!("zinniad/{} {module_name}", env!("CARGO_PKG_VERSION")),
        wallet_address: config.wallet_address,
        reporter: Rc::new(StationReporter::new(
            state_file,
            Duration::from_millis(200),
            module_name.into(),
        )),
        lassie_daemon,
        module_root: Some(module_root),
        no_color: true,
        is_tty: false,
        rng_seed: None,
    };

    // TODO: handle module exit and restart it
    // https://github.com/filecoin-station/zinnia/issues/146
    log::info!("Starting module {main_module}");
    run_js_module(&main_module, &runtime_config).await?;

    Ok(())
}

fn setup_logger() {
    let mut builder = env_logger::Builder::new();
    builder.filter_level(log::LevelFilter::Info);
    builder.parse_default_env();
    builder.init();
}

fn exit_with_error(error: Error) {
    let error_string = format!("{error:?}");
    let error_code = 1;

    log::error!("{}", error_string.trim_start_matches("error: "));
    std::process::exit(error_code);
}

fn setup_lassie_tempdir(lassie_temp_dir: &Path) -> Result<()> {
    if !lassie_temp_dir.is_dir() {
        log::debug!("Creating Lassie tempdir {:?}", lassie_temp_dir,);
        fs::create_dir_all(lassie_temp_dir)?;
        return Ok(());
    }

    log::debug!(
        "Cleaning left-over files in Lassie tempdir {:?}",
        lassie_temp_dir
    );

    let lassie_files = fs::read_dir(lassie_temp_dir).with_context(|| {
        format!(
            "cannot list files in Lassie's temp dir {:?}",
            lassie_temp_dir
        )
    })?;

    for ent in lassie_files {
        match ent {
            Err(err) => log::warn!("Cannot parse dir entry in Lassie tempdir: {:?}", err),
            Ok(f) => {
                let p = f.path();
                // We are assuming that Lassie creates only files, never subdirectories
                log::trace!("Removing Lassie temp file {:?}", p);
                if let Err(err) = fs::remove_file(&p) {
                    log::warn!("Cannot remove Lassie temp file {:?}: {:?}", p, err)
                }
            }
        };
    }

    Ok(())
}
