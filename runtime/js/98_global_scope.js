// ZINNIA VERSION: Copyright 2023-2025 Space Meridian. All rights reserved. MIT OR Apache-2.0 license.
// ORIGINAL WORK: Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.
// https://github.com/denoland/deno/blob/86785f21194460d713276dca2/runtime/js/98_global_scope.js

import { core } from "ext:core/mod.js";

import * as event from "ext:deno_web/02_event.js";
import * as timers from "ext:deno_web/02_timers.js";
import * as base64 from "ext:deno_web/05_base64.js";
import * as encoding from "ext:deno_web/08_text_encoding.js";
import * as console from "ext:deno_console/01_console.js";
import * as compression from "ext:deno_web/14_compression.js";
import * as performance from "ext:deno_web/15_performance.js";
import * as crypto from "ext:deno_crypto/00_crypto.js";
import * as url from "ext:deno_url/00_url.js";
import * as urlPattern from "ext:deno_url/01_urlpattern.js";
import * as headers from "ext:deno_fetch/20_headers.js";
import * as streams from "ext:deno_web/06_streams.js";
// import * as fileReader from "ext:deno_web/10_filereader.js";
// import * as file from "ext:deno_web/09_file.js";
import * as webSocket from "ext:deno_websocket/01_websocket.js";
import * as formData from "ext:deno_fetch/21_formdata.js";
import * as request from "ext:deno_fetch/23_request.js";
import * as response from "ext:deno_fetch/23_response.js";
import * as eventSource from "ext:deno_fetch/27_eventsource.js";
import * as messagePort from "ext:deno_web/13_message_port.js";
import * as webidl from "ext:deno_webidl/00_webidl.js";
import { DOMException } from "ext:deno_web/01_dom_exception.js";
import * as abortSignal from "ext:deno_web/03_abort_signal.js";
import * as httpClient from "ext:deno_fetch/22_http_client.js";

import * as globalInterfaces from "ext:deno_web/04_global_interfaces.js";

import * as fetch from "ext:zinnia_runtime/fetch.js";
import { zinniaNs, log } from "ext:zinnia_runtime/90_zinnia_apis.js";

// https://developer.mozilla.org/en-US/docs/Web/API/WindowOrWorkerGlobalScope
const windowOrWorkerGlobalScope = {
  AbortController: core.propNonEnumerable(abortSignal.AbortController),
  AbortSignal: core.propNonEnumerable(abortSignal.AbortSignal),
  // Intentionally disabled until we need this.
  // See https://github.com/CheckerNetwork/zinnia/issues/46
  // Blob: core.propNonEnumerable(file.Blob),
  ByteLengthQueuingStrategy: core.propNonEnumerable(streams.ByteLengthQueuingStrategy),
  CloseEvent: core.propNonEnumerable(event.CloseEvent),
  CompressionStream: core.propNonEnumerable(compression.CompressionStream),
  CountQueuingStrategy: core.propNonEnumerable(streams.CountQueuingStrategy),
  CryptoKey: core.propNonEnumerable(crypto.CryptoKey),
  CustomEvent: core.propNonEnumerable(event.CustomEvent),
  DecompressionStream: core.propNonEnumerable(compression.DecompressionStream),
  DOMException: core.propNonEnumerable(DOMException),
  ErrorEvent: core.propNonEnumerable(event.ErrorEvent),
  Event: core.propNonEnumerable(event.Event),
  EventTarget: core.propNonEnumerable(event.EventTarget),
  // Intentionally disabled until we need this.
  // See https://github.com/CheckerNetwork/zinnia/issues/46
  // File: core.propNonEnumerable(file.File),
  // FileReader: core.propNonEnumerable(fileReader.FileReader),
  FormData: core.propNonEnumerable(formData.FormData),
  Headers: core.propNonEnumerable(headers.Headers),
  // Intentionally disabled until we need this.
  // See https://github.com/CheckerNetwork/zinnia/issues/724
  // ImageData: core.propNonEnumerable(imageData.ImageData),
  // ImageBitmap: core.propNonEnumerableLazyLoaded((image) => image.ImageBitmap, loadImage),
  MessageEvent: core.propNonEnumerable(event.MessageEvent),
  Performance: core.propNonEnumerable(performance.Performance),
  PerformanceEntry: core.propNonEnumerable(performance.PerformanceEntry),
  PerformanceMark: core.propNonEnumerable(performance.PerformanceMark),
  PerformanceMeasure: core.propNonEnumerable(performance.PerformanceMeasure),
  PromiseRejectionEvent: core.propNonEnumerable(event.PromiseRejectionEvent),
  ProgressEvent: core.propNonEnumerable(event.ProgressEvent),
  ReadableStream: core.propNonEnumerable(streams.ReadableStream),
  ReadableStreamDefaultReader: core.propNonEnumerable(streams.ReadableStreamDefaultReader),
  Request: core.propNonEnumerable(request.Request),
  Response: core.propNonEnumerable(response.Response),
  TextDecoder: core.propNonEnumerable(encoding.TextDecoder),
  TextEncoder: core.propNonEnumerable(encoding.TextEncoder),
  TextDecoderStream: core.propNonEnumerable(encoding.TextDecoderStream),
  TextEncoderStream: core.propNonEnumerable(encoding.TextEncoderStream),
  TransformStream: core.propNonEnumerable(streams.TransformStream),
  URL: core.propNonEnumerable(url.URL),
  URLPattern: core.propNonEnumerable(urlPattern.URLPattern),
  URLSearchParams: core.propNonEnumerable(url.URLSearchParams),
  WebSocket: core.propNonEnumerable(webSocket.WebSocket),
  // Intentionally disabled until we need this.
  // https://github.com/CheckerNetwork/zinnia/issues/725
  // MessageChannel: core.propNonEnumerable(messagePort.MessageChannel),
  MessagePort: core.propNonEnumerable(messagePort.MessagePort),
  // Intentionally disabled until we need this.
  // https://github.com/CheckerNetwork/zinnia/issues/725
  // Worker: core.propNonEnumerable(worker.Worker),
  WritableStream: core.propNonEnumerable(streams.WritableStream),
  WritableStreamDefaultWriter: core.propNonEnumerable(streams.WritableStreamDefaultWriter),
  WritableStreamDefaultController: core.propNonEnumerable(streams.WritableStreamDefaultController),
  ReadableByteStreamController: core.propNonEnumerable(streams.ReadableByteStreamController),
  ReadableStreamBYOBReader: core.propNonEnumerable(streams.ReadableStreamBYOBReader),
  ReadableStreamBYOBRequest: core.propNonEnumerable(streams.ReadableStreamBYOBRequest),
  ReadableStreamDefaultController: core.propNonEnumerable(streams.ReadableStreamDefaultController),
  TransformStreamDefaultController: core.propNonEnumerable(
    streams.TransformStreamDefaultController,
  ),
  atob: core.propWritable(base64.atob),
  btoa: core.propWritable(base64.btoa),
  // Intentionally disabled until we need this.
  // See https://github.com/CheckerNetwork/zinnia/issues/724
  // createImageBitmap: core.propWritableLazyLoaded((image) => image.createImageBitmap, loadImage),
  clearInterval: core.propWritable(timers.clearInterval),
  clearTimeout: core.propWritable(timers.clearTimeout),
  console: core.propNonEnumerable(new console.Console(log)),
  crypto: core.propReadOnly(crypto.crypto),
  Crypto: core.propNonEnumerable(crypto.Crypto),
  SubtleCrypto: core.propNonEnumerable(crypto.SubtleCrypto),
  fetch: core.propWritable(fetch.fetch),
  EventSource: core.propWritable(eventSource.EventSource),
  performance: core.propWritable(performance.performance),
  reportError: core.propWritable(event.reportError),
  setInterval: core.propWritable(timers.setInterval),
  setTimeout: core.propWritable(timers.setTimeout),
  structuredClone: core.propWritable(messagePort.structuredClone),
  // Branding as a WebIDL object
  [webidl.brand]: core.propNonEnumerable(webidl.brand),
  createHttpClient: core.propWritable(httpClient.createHttpClient),
};

const mainRuntimeGlobalProperties = {
  // Location: location.locationConstructorDescriptor,
  // location: location.locationDescriptor,
  Window: globalInterfaces.windowConstructorDescriptor,
  self: core.propGetterOnly(() => globalThis),
  // Navigator: core.propNonEnumerable(Navigator),
  // navigator: core.propGetterOnly(() => navigator),
  // alert: core.propWritable(prompt.alert),
  // confirm: core.propWritable(prompt.confirm),
  // prompt: core.propWritable(prompt.prompt),
  // localStorage: core.propGetterOnly(webStorage.localStorage),
  // sessionStorage: core.propGetterOnly(webStorage.sessionStorage),
  // Storage: core.propNonEnumerable(webStorage.Storage),
  Zinnia: core.propReadOnly(zinniaNs),
};

// prettier-ignore
export {
  windowOrWorkerGlobalScope,
  mainRuntimeGlobalProperties,
};
