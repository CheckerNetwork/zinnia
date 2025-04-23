import { filename } from "./module_fixtures/import_meta_filename.js";

console.log("imported module filename:", filename);
console.log("import.meta.filename:", import.meta.filename);
console.log("import.meta.dirname:", import.meta.dirname);
console.log("error stack:", new Error().stack.split("\n")[1].trim());
