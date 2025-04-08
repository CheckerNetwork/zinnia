// ZINNIA VERSION: Copyright 2023 Protocol Labs. All rights reserved. MIT OR Apache-2.0 license.
// ORIGINAL WORK: Copyright 2018-2025 the Deno authors. MIT license.
// https://github.com/denoland/deno/blob/86785f21194460d713276dca2/runtime/js/06_util.js

import { primordials } from "ext:core/mod.js";
// TODO: op_bootstrap_log_level,
// import { op_bootstrap_log_level } from "ext:core/ops";
const { SafeArrayIterator } = primordials;

// WARNING: Keep this in sync with Rust (search for LogLevel)
const LogLevel = {
  Error: 1,
  Warn: 2,
  Info: 3,
  Debug: 4,
};

const logSource = "JS";

const logLevel = () => 3;

/*
let logLevel_ = null;
function logLevel() {
  if (logLevel_ === null) {
    logLevel_ = op_bootstrap_log_level() || 3;
  }
  return logLevel_;
}
*/

function log(...args) {
  if (logLevel() >= LogLevel.Debug) {
    // if we destructure `console` off `globalThis` too early, we don't bind to
    // the right console, therefore we don't log anything out.
    globalThis.console.error(`DEBUG ${logSource} -`, ...new SafeArrayIterator(args));
  }
}

export { log };
