// Offline tests communicating over WebSockets with a custom server

use anyhow::{Context, Result};
use assert_fs::prelude::*;
use std::rc::Rc;
use zinnia_runtime::{anyhow, deno_core, run_js_module, BootstrapOptions, RecordingReporter};

mod lassie_daemon;
use lassie_daemon::lassie_daemon;

mod websocket_echo_server;
use websocket_echo_server::WebSocketEchoServer;

const USER_AGENT: &str = "zinnia_websockets_tests";

#[tokio::test]
async fn websockets_echo_client() -> Result<()> {
    let _ = env_logger::builder().is_test(true).try_init();

    let echo_server = WebSocketEchoServer::create().await?;
    let server_port = echo_server.port()?;
    tokio::spawn(async move { echo_server.run().await });

    let mod_js = assert_fs::NamedTempFile::new("websockets-test.js")?;
    mod_js.write_str(
        &r#"
import { assertStrictEquals } from "zinnia:assert";
const socket = new WebSocket("ws://127.0.0.1:SERVER_PORT");
const { promise, resolve, reject } = Promise.withResolvers();

socket.addEventListener("open", (event) => {
  socket.send("Hello Server!");
});

socket.addEventListener("message", (event) => {
  resolve(event.data)
});

socket.addEventListener("error", (event) => {
 reject(new Error(`WebSocket error: ${event}`))
});

const result = await promise;
socket.close();
assertStrictEquals(result, "Hello Server!");
"#
        .replace(&"SERVER_PORT", &server_port.to_string()),
    )?;

    let main_module = deno_core::resolve_path(
        &mod_js.to_string_lossy(),
        &std::env::current_dir().context("unable to get current working directory")?,
    )?;
    let config = BootstrapOptions::new(
        USER_AGENT.into(),
        Rc::new(RecordingReporter::new()),
        lassie_daemon(),
        None,
    );
    run_js_module(&main_module, &config).await?;
    // the test passes when the JavaScript code does not throw
    Ok(())
}
