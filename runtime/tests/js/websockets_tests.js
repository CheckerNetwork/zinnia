// These tests are communicating with external WebSocket servers and will fail when the computer is offline.

import { test } from "zinnia:test";
import { assertMatch, assertStrictEquals } from "zinnia:assert";

test("websockets over TLS", async () => {
  // See https://websocket.org/tools/websocket-echo-server/
  // > We run a free very simple endpoint server with support for websockets
  // > and server-sent events (SSE) so that you can test your websocket and
  // > SSE clients easily.
  const socket = new WebSocket("wss://echo.websocket.org/");

  await once(socket, "open");
  const [welcome] = await once(socket, "message");
  // console.log("welcome", welcome);
  assertMatch(welcome.data, /^Request served by/);

  socket.send("Hello Server!");
  const [echo] = await once(socket, "message");
  // console.log("echo", echo);
  assertStrictEquals(echo.data, "Hello Server!");

  socket.close();
});

function once(socket, eventName) {
  const { promise, resolve, reject } = Promise.withResolvers();
  let onEvent, onError;
  onEvent = (...args) => {
    socket.removeEventListener("error", onError);
    resolve(args);
  };
  onError = (error) => {
    socket.removeEventListener(eventName, onEvent);
    reject(error);
  };

  socket.addEventListener(eventName, onEvent, { once: true });
  socket.addEventListener("error", onError, { once: true });

  return promise;
}
