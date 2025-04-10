mod args;

use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

use args::{CliArgs, Commands};
use clap::Parser;

use zinnia_runtime::anyhow::{Context, Result};
use zinnia_runtime::fmt_errors::format_js_error;
use zinnia_runtime::{
    any_and_jserrorbox_downcast_ref, colors, lassie, lassie_config, resolve_path, run_js_module,
    AnyError, BootstrapOptions, ConsoleReporter, CoreError,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    #[cfg(windows)]
    colors::enable_ansi(); // For Windows 10

    match main_impl().await {
        Ok(_) => (),
        Err(err) => exit_for_error(err),
    }
}

async fn main_impl() -> Result<()> {
    let cli_args = CliArgs::parse_from(std::env::args());
    match cli_args.command {
        Commands::Run { file } => {
            run_module(file).await?;

            Ok(())
        }
    }
}

#[allow(dead_code)]
struct RunOutput {
    module_output: (),
    // for testing
    lassie_daemon: Arc<lassie::Daemon>,
}

async fn run_module(file: String) -> Result<RunOutput> {
    let main_module = resolve_path(
        &file,
        &std::env::current_dir().context("unable to get current working directory")?,
    )?;

    let lassie_daemon = Arc::new(
        lassie::Daemon::start(lassie::DaemonConfig {
            // This configuration applies to `zinnia` CLI only. The `zinniad` daemon running
            // inside Station uses a different temp_dir config based on the env var
            // `CACHE_ROOT` provided by the Station.
            //
            // By default, Lassie stores its temporary files in the system temp directory.
            // That's good enough for now. We can improve this later based on user feedback,
            // for example:
            // - we can honour CACHE_ROOT
            // - we can default to something like
            //   `~/.cache/zinnia/lassie` on Unix,
            //   `%APPLOCALDATA%\zinnia\lassie' on Windows.
            //
            // Important: if we tell Lassie to use a specific temp dir that's not
            // automatically cleaned by the operating system, we will need to clean any
            // leftover files ourselves. See the GH issue for deleting leftover files
            // when `zinniad` starts: https://github.com/filecoin-station/zinnia/issues/245
            temp_dir: None,
            ..lassie_config()
        })
        .context("cannot initialize the IPFS retrieval client Lassie")?,
    );

    let runtime_config = BootstrapOptions {
        zinnia_version: env!("CARGO_PKG_VERSION"),
        ..BootstrapOptions::new(
            format!("zinnia/{}", env!("CARGO_PKG_VERSION")),
            Rc::new(ConsoleReporter::new(Duration::from_millis(500))),
            Arc::clone(&lassie_daemon),
            None,
        )
    };

    #[allow(clippy::let_unit_value)]
    let module_output = run_js_module(&main_module, &runtime_config).await?;

    Ok(RunOutput {
        module_output,
        lassie_daemon,
    })
}

// Inspired by exit_for_error from Deno's `cli/main.rs`
// https://github.com/denoland/deno/blob/main/cli/main.rs

fn exit_with_message(message: &str, code: i32) -> ! {
    eprintln!(
        "{}: {}",
        colors::red_bold("error"),
        message.trim_start_matches("error: ")
    );
    zinnia_runtime::exit(code);
}

fn exit_for_error(error: AnyError) -> ! {
    let mut error_string = format!("{error:?}");
    let error_code = 1;

    if let Some(CoreError::Js(e)) = any_and_jserrorbox_downcast_ref::<CoreError>(&error) {
        error_string = format_js_error(e);
    }

    exit_with_message(&error_string, error_code);
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[tokio::test]
    async fn lassie_auth_is_configured() {
        // Step 1: execute `zinnia run` with a dummy module that does nothing
        let mod_js =
            assert_fs::NamedTempFile::new("dummy.js").expect("cannot create temp dummy.js");

        mod_js
            .write_str("/* no-op */")
            .expect("cannot write to dummy.js");

        let RunOutput { lassie_daemon, .. } =
            run_module(mod_js.path().to_string_lossy().to_string())
                .await
                .expect("cannot run dummy.js");

        assert!(
            lassie_daemon.access_token().is_some(),
            "lassie_daemon access_token was not set"
        );

        // Make a retrieval request to Lassie but do not provide any access token
        let mut stream =
            tokio::net::TcpStream::connect(format!("127.0.0.1:{}", lassie_daemon.port()))
                .await
                .expect("cannot connect to Lassie HTTP daemon");

        stream
            .write_all(
                concat!(
                "GET /ipfs/bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni HTTP/1.1\n",
                "Host: 127.0.0.1\n",
                "\n",
                )
                .as_bytes(),
            )
            .await
            .expect("cannot write HTTP request");

        let status = BufReader::new(stream)
            .lines()
            .next_line()
            .await
            .expect("cannot read the first line of the HTTP response")
            .expect("server returned at least one line");

        assert_eq!(status, "HTTP/1.1 401 Unauthorized")
    }
}
