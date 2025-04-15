import { assertEquals } from "zinnia:assert";

import data from "./data.json" with { type: "json" };
assertEquals(data, { name: "Jane Smith", age: 30, email: "jane.smith@example.com" });
