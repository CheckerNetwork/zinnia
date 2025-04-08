// ZINNIA VERSION: Copyright 2023 Protocol Labs. All rights reserved. MIT OR Apache-2.0 license.
// ORIGINAL WORK: Copyright 2018-2025 the Deno authors. MIT license.
// https://github.com/denoland/deno/blob/86785f21194460d713276dca2/runtime/js/99_main.js

// Removes the `__proto__` for security reasons.
// https://tc39.es/ecma262/#sec-get-object.prototype.__proto__
delete Object.prototype.__proto__;

// Remove Intl.v8BreakIterator because it is a non-standard API.
delete Intl.v8BreakIterator;

import { core, primordials } from "ext:core/mod.js";
import { op_set_format_exception_callback } from "ext:core/ops";
const {
  Error,
  ErrorPrototype,
  ObjectDefineProperties,
  ObjectPrototypeIsPrototypeOf,
  ObjectSetPrototypeOf,
  Symbol,
} = primordials;
const { isNativeError } = core;
import * as util from "ext:zinnia_runtime/06_util.js";
import * as event from "ext:deno_web/02_event.js";
import * as version from "ext:zinnia_runtime/01_version.ts";
import {
  getDefaultInspectOptions,
  getStderrNoColor,
  inspectArgs,
  quoteString,
  // setNoColorFns,
} from "ext:deno_console/01_console.js";
import * as performance from "ext:deno_web/15_performance.js";
import * as fetch from "ext:deno_fetch/26_fetch.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import { SymbolDispose, SymbolMetadata } from "ext:deno_web/00_infra.js";
import { bootstrap as bootstrapOtel } from "ext:deno_telemetry/telemetry.ts";

import {
  mainRuntimeGlobalProperties,
  windowOrWorkerGlobalScope,
} from "ext:zinnia_runtime/98_global_scope.js";
import { setLassieConfig } from "ext:zinnia_runtime/fetch.js";

// deno-lint-ignore prefer-primordials
if (Symbol.metadata) {
  throw "V8 supports Symbol.metadata now, no need to shim it";
}

ObjectDefineProperties(Symbol, {
  dispose: {
    __proto__: null,
    value: SymbolDispose,
    enumerable: false,
    writable: false,
    configurable: false,
  },
  metadata: {
    __proto__: null,
    value: SymbolMetadata,
    enumerable: false,
    writable: false,
    configurable: false,
  },
});

// https://docs.rs/log/latest/log/enum.Level.html
const LOG_LEVELS = {
  error: 1,
  warn: 2,
  info: 3,
  debug: 4,
  trace: 5,
};

let globalThis_;

function formatException(error) {
  if (isNativeError(error) || ObjectPrototypeIsPrototypeOf(ErrorPrototype, error)) {
    return null;
  } else if (typeof error == "string") {
    return `Uncaught ${inspectArgs([quoteString(error, getDefaultInspectOptions())], {
      colors: !getStderrNoColor(),
    })}`;
  } else {
    return `Uncaught ${inspectArgs([error], { colors: !getStderrNoColor() })}`;
  }
}

core.registerErrorBuilder("DOMExceptionOperationError", function DOMExceptionOperationError(msg) {
  return new DOMException(msg, "OperationError");
});
core.registerErrorBuilder(
  "DOMExceptionQuotaExceededError",
  function DOMExceptionQuotaExceededError(msg) {
    return new DOMException(msg, "QuotaExceededError");
  },
);
core.registerErrorBuilder(
  "DOMExceptionNotSupportedError",
  function DOMExceptionNotSupportedError(msg) {
    return new DOMException(msg, "NotSupported");
  },
);
core.registerErrorBuilder("DOMExceptionNetworkError", function DOMExceptionNetworkError(msg) {
  return new DOMException(msg, "NetworkError");
});
core.registerErrorBuilder("DOMExceptionAbortError", function DOMExceptionAbortError(msg) {
  return new DOMException(msg, "AbortError");
});
core.registerErrorBuilder(
  "DOMExceptionInvalidCharacterError",
  function DOMExceptionInvalidCharacterError(msg) {
    return new DOMException(msg, "InvalidCharacterError");
  },
);
core.registerErrorBuilder("DOMExceptionDataError", function DOMExceptionDataError(msg) {
  return new DOMException(msg, "DataError");
});
core.registerErrorBuilder(
  "DOMExceptionInvalidStateError",
  function DOMExceptionInvalidStateError(msg) {
    return new DOMException(msg, "InvalidStateError");
  },
);

function runtimeStart({ zinniaVersion, v8Version, lassieUrl, lassieAuth }) {
  core.setWasmStreamingCallback(fetch.handleWasmStreaming);
  core.setReportExceptionCallback(event.reportException);
  op_set_format_exception_callback(formatException);
  version.setVersions(zinniaVersion, v8Version);
  // core.setBuildInfo(target);

  // FIXME: figure out log levels
  // util.setLogDebug(runtimeOptions.debugFlag, source);
  // FIXME: rework to lazy load, see
  // https://github.com/denoland/deno/commit/1ef617e8f3d48098e69e222b6eb6fe981aeca1c3
  // FIXME: figure out color detection
  // https://github.com/denoland/deno/blob/6d33141d8dd88123b76476e4c91e608919f6736c/runtime/ops/bootstrap.rs#L130-L133
  // https://github.com/denoland/deno_terminal/
  // setNoColorFn(() => runtimeOptions.noColor || !runtimeOptions.isTty);

  setLassieConfig(lassieUrl, lassieAuth);
}

let hasBootstrapped = false;
// Set up global properties shared by main and worker runtime.
ObjectDefineProperties(globalThis, windowOrWorkerGlobalScope);

function bootstrapMainRuntime(runtimeOptions) {
  if (hasBootstrapped) {
    throw new Error("Worker runtime already bootstrapped");
  }

  performance.setTimeOrigin();
  globalThis_ = globalThis;

  // Remove bootstrapping data from the global scope
  delete globalThis.__bootstrap;
  delete globalThis.bootstrap;
  hasBootstrapped = true;

  ObjectDefineProperties(globalThis, mainRuntimeGlobalProperties);
  ObjectSetPrototypeOf(globalThis, Window.prototype);

  bootstrapOtel([
    0, // tracingEnabled
    0, // metricsEnabled
    0, // consoleConfig: ignore
  ]);

  if (runtimeOptions.inspectFlag) {
    core.wrapConsole(globalThis.console, core.v8Console);
  }

  event.setEventTargetData(globalThis);
  event.saveGlobalThisReference(globalThis);

  runtimeStart(runtimeOptions);

  ObjectDefineProperties(globalThis.Zinnia, {
    walletAddress: core.propReadOnly(runtimeOptions.walletAddress),
    stationId: core.propReadOnly(runtimeOptions.stationId),
  });

  // delete `Deno` global
  delete globalThis.Deno;

  util.log("args", runtimeOptions.args);
}

globalThis.bootstrap = {
  mainRuntime: bootstrapMainRuntime,
};

// Workaround to silence Deno runtime assert
// "Following modules were not evaluated; make sure they are imported from other code"
import "ext:zinnia_runtime/internals.js";
import "ext:zinnia_runtime/test.js";
import "ext:zinnia_runtime/vendored/asserts.bundle.js";
import "ext:deno_web/16_image_data.js";
import "ext:deno_web/10_filereader.js";
import "ext:zinnia_runtime/06_util.js";
