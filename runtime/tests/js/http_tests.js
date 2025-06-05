import { test } from "zinnia:test";
import { assert, assertEquals } from "zinnia:assert";

test("Zinnia.createHttpClient", async () => {
  const client = Zinnia.createHttpClient({
    localAddress: "127.0.0.2",
  });
  assert(client);
  const res = await fetch("https://example.com/", { client });
  assertEquals(res.status, 200);
});

