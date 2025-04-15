import { test } from "zinnia:test";
import { assert, assertEquals } from "zinnia:assert";

test("AbortController", () => {
  assertEquals(typeof AbortController, "function", "typeof AbortController");
  assertEquals(AbortController.name, "AbortController", "AbortController.name");
});

test("atob & btoa", () => {
  assertEquals(btoa("some text"), "c29tZSB0ZXh0", `btoa("some text)`);
  assertEquals(atob("c29tZSB0ZXh0"), "some text", `atob("c29tZSB0ZXh0")`);
});

test("TextEncoder", () => {
  const encoder = new TextEncoder();
  const bytes = encoder.encode("€");
  assertEquals(Array.from(bytes.values()), [226, 130, 172]);
});

test("TextDecoder", () => {
  let decoder = new TextDecoder();
  let bytes = new Uint8Array([226, 130, 172]);
  let text = decoder.decode(bytes);
  assertEquals(text, "€");
});

test("URL", () => {
  const url = new URL("https://filstation.app");
  assertEquals(url.host, "filstation.app");
});

test("URL.parse()", () => {
  // Testcases from https://developer.mozilla.org/en-US/docs/Web/API/URL/parse_static

  // Relative reference to a valid base URL
  assertEquals(
    URL.parse("en-US/docs", "https://developer.mozilla.org")?.href,
    "https://developer.mozilla.org/en-US/docs",
  );

  // Invalid base URL (missing colon)
  assertEquals(URL.parse("en-US/docs", "https//developer.mozilla.org"), null);
});

test("Float16Array", () => {
  const float16 = new Float16Array([42]);
  assertEquals(float16[0], 42);
  assertEquals(float16.length, 1);
  assertEquals(float16.byteLength, 2);
  assertEquals(float16.BYTES_PER_ELEMENT, 2);
});

test("import.meta.filename", () => {
  const value = import.meta.filename.replace("\\", "/");
  const expectedSuffix = "/runtime/tests/js/webapis_tests.js";
  assert(
    value.endsWith(expectedSuffix),
    `Expected import.meta.filename to end with ${expectedSuffix}. Actual value: ${value}`,
  );
});

test("import.meta.dirname", () => {
  const value = import.meta.dirname.replace("\\", "/");
  const expectedSuffix = "/runtime/tests/js";
  assert(
    value.endsWith(expectedSuffix),
    `Expected import.meta.filename to end with ${expectedSuffix}. Actual value: ${value}`,
  );
});
