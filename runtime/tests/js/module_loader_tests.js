import { test } from "zinnia:test";
import { assertEquals, assertMatch, assertRejects } from "zinnia:assert";

test("dynamically import file next to the main module file", async () => {
  const { KEY } = await import("./empty_module.js");
  assertEquals(KEY, 1);
});

test("statically import a file inside the module tree", async () => {
  // lib contains `import` from `./log.js`, check that it's allowed
  // also check that we can import from nested directories
  await import("./module_fixtures/lib.js");
});

test("can import files outside the main module directory", async () => {
  await assertRejects(() => import("../../js/99_main.js"));
});

test("cannot import files over http", async () => {
  let err = await assertRejects(() => import("https://deno.land/std@0.181.0/version.ts"));
  assertMatch(err.message, /Zinnia can import local modules only/);
});

test("can import WASM files as modules", async () => {
  const { add } = await import("./module_fixtures/math.wasm");
  const result = add(2, 3);
  assertEquals(result, 5);
});
