import { test } from "zinnia:test";
import { assertEquals, assertStringIncludes } from "zinnia:assert";

// We must not leak the filesystem structure when the code is running inside a sandboxed environment.
// Only path relative to the project root is available to the module.

test("import.meta.filename is relative to the module root", () => {
  const value = import.meta.filename.replace("\\", "/");
  assertEquals(value, "/source_code_paths.js", "import.meta.filename");
});

test("import.meta.dirname is relative to the module root", () => {
  const value = import.meta.dirname.replace("\\", "/");
  assertEquals(value, "/", "import.meta.dirname");
});

test("stack trace contains file paths relative to the module root", () => {
  const thisLine = new Error().stack.split("\n")[1];
  assertStringIncludes(thisLine, "(file:///source_code_paths.js");
});
