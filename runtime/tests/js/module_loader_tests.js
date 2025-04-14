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

test("import JSON files using dynamic `import()` (with attributes)", async () => {
  const { default: data } = await import("./module_fixtures/data.json", { with: { type: "json" } });
  assertEquals(data, { name: "Jane Smith", age: 30, email: "jane.smith@example.com" });
});

test("import JSON files using dynamic `import()` (no attributes)", async () => {
  const err = await assertRejects(async () => import("./module_fixtures/data.json"));
  assertMatch(err.message, /"type": "json" attribute/);
  assertEquals(err.code, "ERR_MODULE_NOT_FOUND");
});

test("import JSON files using static `import` (with attributes)", async () => {
  await import("./module_fixtures/import-json-with-attributes.js");
});

test("import JSON files using static `import` (no attributes)", async () => {
  const err = await assertRejects(
    async () => import("./module_fixtures/import-json-no-attributes.js"),
  );

  assertMatch(err.message, /"type": "json" attribute/);
  assertEquals(err.code, "ERR_MODULE_NOT_FOUND");
});

test("cannot import files over http", async () => {
  let err = await assertRejects(() => import("https://deno.land/std@0.181.0/version.ts"));
  assertMatch(err.message, /Zinnia can import local modules only/);
});
