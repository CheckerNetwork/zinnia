import { test } from "zinnia:test";
import { assert, assertEquals, assertRejects } from "zinnia:assert";

test("fetch", async () => {
  const res = await fetch("https://google.com/");
  assertEquals(res.status, 200);
  const text = await res.text();
  assertEquals(typeof text, "string");
  assert(text.includes("<body"));
});

test("fetch from filesystem", async () => {
  const err = await assertRejects(async () => fetch("file:///etc/passwd"));
  assertEquals(err.message, "NetworkError when attempting to fetch resource");
});

test("FormData", async () => {
  const formData = new FormData();
  formData.append("name", "value");
});

test("Headers", async () => {
  const headers = new Headers();
  headers.append("name", "value");
});

test("ProgressEvent", async () => {
  const event = new ProgressEvent();
  assertEquals(event.total, 0);
});

test("Request", async () => {
  const request = new Request("https://example.com/");
  await request.arrayBuffer();
});

test("Response", async () => {
  const response = new Response();
  await response.arrayBuffer();
});

test("Response.bytes()", async () => {
  const response = new Response();
  await response.bytes();
});
