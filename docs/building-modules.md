# Building Modules

A Station Module is a long-running process that's performing jobs like network probes, content
delivery, and computation.

Zinnia provides a JavaScript runtime with a set of platform APIs allowing modules to interact with
the outside world.

In the long run, we want Zinnia to be aligned with the Web APIs as much as feasible.

For the shorter term, we are going to take shortcuts to deliver a useful platform quickly.

## Getting Started

If you haven't done so, then install `zinnia` CLI per
[our instructions](../cli/README.md#installation).

Using your favourite text editor, create a file called `module.js` with the following content:

```js
console.log("Hello universe!");
```

Open the terminal and run the module by using `zinnia run` command:

```
$ zinnia run module.js
Hello universe!
```

See [example modules](../examples) for more advanced examples.

## Table of Contents

- [Importing JavaScript Modules](#importing-javascript-modules)
- [Working with WebAssembly](#working-with-webassembly)
- [Platform APIs](#platform-apis)
- [Testing Guide](#testing-guide)

## Importing JavaScript Modules

Zinnia supports ES Modules (also known as
[JavaScript Modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules)).

### Sandboxing in Filecoin Station

Filecoin Station limits module imports to files in the root directory of the Zinnia module being
executed.

This limitation DOES NOT apply when running your code using `zinnia run`.

Consider the following directory layout:

```
src
  my-module
    util
      log.js
      helpers.js
    main.js
    lib.js
  other
    code.js
```

When you execute `zinniad src/my-module/main.js`:

- In `main.js`, you can import any JavaScript file inside `src/my-module` directory and its
  subdirectories (e.g. `src/my-module/util`).
- The same restriction applies transitively to other imported files too.

Example:

```js
// These imports are allowed in `main.js`
import { processJob } from "./lib.js";
import { log } from "./util/log.js";

// This will be rejected at runtime
import * as code from "../other/code.js";

// This will work in `util/log.js`
import { format } from "../lib.js";

// This will be rejected
import * as code from "../../other/code.js";
```

## Working with WebAssembly

Zinnia can directly import functions exported by WebAssembly modules.

```js
import { add } from "./math.wasm";
console.log(add(1, 2));
```

We are looking for feedback from the community to improve the WASM support. Join the discussion on
GitHub in [zinnia#74](https://github.com/CheckerNetwork/zinnia/issues/74).

## Platform APIs

- [Standard JavaScript APIs](#standard-javascript-apis)
- [Web APIs](#web-apis)
- [Unsupported Web APIs](#unsupported-web-apis)
- [libp2p](#libp2p)
- [Integration with Filecoin Station](#integration-with-filecoin-station)
- [IPFS retrieval client](#ipfs-retrieval-client)
- [Miscelaneous APIs]()

### Standard JavaScript APIs

Zinnia provides all standard JavaScript APIs, you can find the full list in
[MDN web docs](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects).

### Web APIs

The following entities are defined in the global scope (`globalThis`).

#### Console Standard

Zinnia implements most of the `console` Web APIs like `console.log`. You can find the full list of
supported methods in [Deno docs](https://deno.land/api@v1.30.3?s=Console) and more details about
individual methods in [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/API/console)

- [console](https://developer.mozilla.org/en-US/docs/Web/API/console)

> Note: Messaged logged using Console APIs will not show in Station UI, they purpose is to help
> Station Module authors to troubleshoot issues. See
> [`Zinnia.activity.info`](#zinniaactivityinfomessage) and
> [`Zinnia.activity.error](#zinniaactivityerrormessage) APIs for reporting information to be shown
> to Station users in the Station Desktop UI.

#### DOM Standard

- [AbortController](https://developer.mozilla.org/en-US/docs/Web/API/AbortController)
- [AbortSignal](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)
- [CustomEvent](https://developer.mozilla.org/en-US/docs/Web/API/CustomEvent)
- [Event](https://developer.mozilla.org/en-US/docs/Web/API/Event)
- [EventTarget](https://developer.mozilla.org/en-US/docs/Web/API/EventTarget)

#### Encoding Standard

- [TextDecoder](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoder)
- [TextEncoder](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoder)
- [TextDecoderStream](https://developer.mozilla.org/en-US/docs/Web/API/TextDecoderStream)
- [TextEncoderStream](https://developer.mozilla.org/en-US/docs/Web/API/TextEncoderStream)

#### Fetch Standard

- [FormData](https://developer.mozilla.org/en-US/docs/Web/API/FormData)
- [Headers](https://developer.mozilla.org/en-US/docs/Web/API/Headers)
- [ProgressEvent](https://developer.mozilla.org/en-US/docs/Web/API/ProgressEvent)
- [Request](https://developer.mozilla.org/en-US/docs/Web/API/Request)
- [Response](https://developer.mozilla.org/en-US/docs/Web/API/Response)
- [fetch](https://developer.mozilla.org/en-US/docs/Web/API/fetch)

#### HTML Standard

- [ErrorEvent](https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent)
- [MessageChannel](https://developer.mozilla.org/en-US/docs/Web/API/MessageChannel)
- [MessageEvent](https://developer.mozilla.org/en-US/docs/Web/API/MessageEvent)
- [MessagePort](https://developer.mozilla.org/en-US/docs/Web/API/MessagePort)
- [PromiseRejectionEvent](https://developer.mozilla.org/en-US/docs/Web/API/PromiseRejectionEvent)
- [atob](https://developer.mozilla.org/en-US/docs/Web/API/atob)
- [btoa](https://developer.mozilla.org/en-US/docs/Web/API/btoa)
- [clearInterval](https://developer.mozilla.org/en-US/docs/Web/API/clearInterval)
- [clearTimeout](https://developer.mozilla.org/en-US/docs/Web/API/clearTimeout)
- [reportError](https://developer.mozilla.org/en-US/docs/Web/API/reportError)
- [setInterval](https://developer.mozilla.org/en-US/docs/Web/API/setInterval)
- [setTimeout](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout)
- [structuredClone](https://developer.mozilla.org/en-US/docs/Web/API/structuredClone)

#### Performance & User Timing

- [Performance](https://developer.mozilla.org/en-US/docs/Web/API/Performance)
- [PerformanceEntry](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry)
- [PerformanceMark](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMark)
- [PerformanceMeasure](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceMeasure)
- [performance](https://developer.mozilla.org/en-US/docs/Web/API/performance)

#### Streams Standard

- [ByteLengthQueuingStrategy](https://developer.mozilla.org/en-US/docs/Web/API/ByteLengthQueuingStrategy)
- [CompressionStream](https://developer.mozilla.org/en-US/docs/Web/API/CompressionStream)
- [CountQueuingStrategy](https://developer.mozilla.org/en-US/docs/Web/API/CountQueuingStrategy)
- [DecompressionStream](https://developer.mozilla.org/en-US/docs/Web/API/DecompressionStream)
- [ReadableByteStreamController](https://developer.mozilla.org/en-US/docs/Web/API/ReadableByteStreamController)
- [ReadableStreamBYOBReader](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBReader)
- [ReadableStreamBYOBRequest](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamBYOBRequest)
- [ReadableStreamDefaultController](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultController)
- [ReadableStreamDefaultReader](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStreamDefaultReader)
- [ReadableStream](https://developer.mozilla.org/en-US/docs/Web/API/ReadableStream)
- [TransformStreamDefaultController](https://developer.mozilla.org/en-US/docs/Web/API/TransformStreamDefaultController)
- [TransformStream](https://developer.mozilla.org/en-US/docs/Web/API/TransformStream)
- [WritableStreamDefaultController](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultController)
- [WritableStreamDefaultWriter](https://developer.mozilla.org/en-US/docs/Web/API/WritableStreamDefaultWriter)
- [WritableStream](https://developer.mozilla.org/en-US/docs/Web/API/WritableStream)

#### URL Standard

- [URL](https://developer.mozilla.org/en-US/docs/Web/API/URL)
- [URLSearchParams](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)
- [URLPattern](https://developer.mozilla.org/en-US/docs/Web/API/URLPattern)

#### Web Cryptography API

- [CryptoKey](https://developer.mozilla.org/en-US/docs/Web/API/CryptoKey)
- [Crypto](https://developer.mozilla.org/en-US/docs/Web/API/Crypto)
- [SubtleCrypto](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto)
- [crypto](https://developer.mozilla.org/en-US/docs/Web/API/crypto)

#### WebSockets Standard

- [CloseEvent](https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent)
- [WebSocket](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket)

#### Web IDL Standard

- [DOMException](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)

### Unsupported Web APIs

#### File API

Tracking issue: n/a

- [Blob](https://developer.mozilla.org/en-US/docs/Web/API/blob)
- [File](https://developer.mozilla.org/en-US/docs/Web/API/File)
- [FileReader](https://developer.mozilla.org/en-US/docs/Web/API/FileReader)

#### Service Workers & Web Workers

Tracking issue: n/a

- [CacheStorage](https://developer.mozilla.org/en-US/docs/Web/API/CacheStorage)
- [Cache](https://developer.mozilla.org/en-US/docs/Web/API/Cache)
- [Worker](https://developer.mozilla.org/en-US/docs/Web/API/Worker)
- [caches](https://developer.mozilla.org/en-US/docs/Web/API/caches)

#### Other

- `XMLHttpRequest` Standard

### libp2p

We removed the initial limited version of a built-in rust-libp2p node. If you are interested in
using libp2p, please upvote the following GitHub issue:
[zinnia#711](https://github.com/CheckerNetwork/zinnia/issues/711)

### Integration with Filecoin Station

#### `Zinnia.stationId`

The associated Station Core's unique identifier (public key)

The value is hard-coded to 88 `0` characters when running the module via `zinnia` CLI.

#### `Zinnia.walletAddress`

The wallet address where to send rewards. When running inside the Station Desktop, this API will
return the address of the Station's built-in wallet.

The value is hard-coded to the Ethereum (FEVM) address `0x000000000000000000000000000000000000dEaD`
when running the module via `zinnia` CLI.

#### `Zinnia.activity.info(message)`

Add a new Activity Log item informing the Station user when things proceed as expected.

Example messages:

```
Saturn Node will try to connect to the Saturn Orchestrator...
Saturn Node is online and connected to 9 peers.
```

#### `Zinnia.activity.error(message)`

Add a new Activity Log informing the Station user about an error state.

Example messages:

```
Saturn Node is not able to connect to the network.
```

#### `Zinnia.jobCompleted()`

Report that a single job was completed.

Call this function every time your module completes a job. It's ok to call it frequently.

### IPFS Retrieval Client

Zinnia provides a built-in IPFS retrieval client making it easy to fetch content-addressed data from
IPFS and Filecoin networks. You can retrieve data for a given CID using the web platform API `fetch`
together with the URL scheme `ipfs://`.

Example:

```js
const response = await fetch("ipfs://bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni");
assert(response.ok);
const data = await response.arrayBuffer();
// data contains binary data in the CAR format
```

> Note: At the moment, Zinnia does not provide any tools for interpreting the returned CAR data. We
> are discussing support for reading UnixFS data in
> [zinnia#245](https://github.com/filecoin-station/zinnia/issues/246).

Under the hood, Zinnia handles `ipfs://bafy...` requests by calling Lassie's HTTP API. You can learn
more about supported parameters (request headers, query string arguments), response headers and
possible error status codes in
[Lassie's HTTP Specification](https://github.com/filecoin-project/lassie/blob/main/docs/HTTP_SPEC.md).
The format of CAR data returned by the retrieval client is described in
[Lassie's Returned CAR Specification](https://github.com/filecoin-project/lassie/blob/main/docs/CAR.md).

#### Timeouts

The IPFS retrieval client is configured to time out after one day. When this happens, the response
body stream is terminated in a way that triggers a reading error.

We strongly recommend to configure a client-side timeout using
[`AbortController`](https://developer.mozilla.org/en-US/docs/Web/API/AbortController) or
[`AbortSignal.timeout()`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout_static).

Example:

```js
const requestUrl = "ipfs://bafybeib36krhffuh3cupjml4re2wfxldredkir5wti3dttulyemre7xkni";
const response = await fetch(requestUrl, {
  signal: AbortSignal.timeout(500), // abort after 500ms
});
// etc.
```

### Miscelaneous APIs

#### `Zinnia.inspect`

Converts the input into a string that has the same format as printed by `console.log()`.

See [Deno.inspect() docs](https://docs.deno.com/api/deno/~/Deno.inspect) for more details.

#### `Zinnia.versions.zinna`

The version of Zinnia runtime, e.g. `"0.11.0"`.

#### `Zinnia.versions.v8`

The version of V8 engine, e.g. `"11.5.150.2"`.

#### `Zinnia.createHttpClient`

Create a custom HttpClient to use with fetch.

See [Deno.createHttpClient() docs](https://docs.deno.com/api/deno/~/Deno.createHttpClient) for more details.

## Testing Guide

Zinnia provides lightweight tooling for writing and running automated tests.

- [Test Runner](#test-runner)
- [Assertions](#assertions)

### Test Runner

The built-in test runner is intentionally minimalistic for now. Let us know what features you would
like us to add!

Example test file (e.g. `test/smoke.test.js`):

```js
import { test } from "zinnia:test";

test("a sync test", () => {
  // run your test
  // throw an error when an assertion fails
});

test("a test can be async too", async () => {
  // run some async code
  // throw an error when an assertion fails
});
```

Notes:

- Calling `test()` DOES NOT run the test immediately. It adds the test to the queue.
- Therefore, you should never `await` the value returned by `test()` .
- The tests are executed sequentially in the order in which they were registered via `test()` calls.

You can run the tests using `zinnia run`:

```bash
❯ zinnia run test/smoke.test.js
```

To run a test suite consisting of multiple test files, create a top-level test suite file and import
individual test files.

For example, you can create `test-all.js` in your project root:

```js
import "./test/smoke.test.js";
import "./test/user.test.js";
// and so on
```

### Assertions

You can use most assertion libraries that are compatible with browsers and Deno, for example
[Chai](https://www.chaijs.com).

Zinnia provides a built-in assertion library based on Deno's `std/assert`.

Example usage:

```js
import { assertEquals } from "zinnia:assert";
assertEquals(true, false);
// ^ throws an error
```

You can find the API documentation at deno.land website:
[https://jsr.io/@std/assert@0.226.0](https://jsr.io/@std/assert@0.226.0)
