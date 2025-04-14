import { test } from "zinnia:test";
import { assertEquals, assertMatch, assertRejects, AssertionError } from "zinnia:assert";

const TEST_CID = "bafkreih25dih6ug3xtj73vswccw423b56ilrwmnos4cbwhrceudopdp5sq";
const FRISBII_INSTANCE_QUERY_PARAMS = "?protocol=http&providers=/dns4/frisbii.fly.dev/https";
const EXPECTED_CAR_SIZE_IN_BYTES = 200;
const EXPECTED_CAR_BASE64 =
  "OqJlcm9vdHOB2CpYJQABVRIg+ujQf1DbvNP91lYQrc1sPfIXGzGulwQbHiIlBueN/ZRndmVyc2lvbgGLAQFVEiD66NB/UNu80/3WVhCtzWw98hcbMa6XBBseIiUG5439lGxhcGlkYXJ5CmJyYXZvCnJlYWwKcGFyZXNpcwpoaWdoYm9ybgpob3JzZQpib3dlbAphc3Npc3QKY29ybmVhCnB5cmUKVU5JRk9STQpza2lpbmcKc3BpcmUKdXBoZWF2ZQpjcnVtcAo=";

test("can retrieve CID content as a CAR file", async () => {
  const requestUrl = `ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`;
  const response = await fetch(requestUrl);
  await assertResponseIsOk(response);

  const payload = await response.arrayBuffer();
  assertEquals(payload.byteLength, EXPECTED_CAR_SIZE_IN_BYTES, "CAR size in bytes");

  const payload_encoded = btoa(String.fromCharCode(...new Uint8Array(payload)));
  assertEquals(payload_encoded, EXPECTED_CAR_BASE64);

  assertEquals(response.url, requestUrl);
});

test("can retrieve IPFS content using URL", async () => {
  const requestUrl = new URL(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`);
  const response = await fetch(requestUrl);
  await assertResponseIsOk(response);

  const payload = await response.arrayBuffer();
  assertEquals(payload.byteLength, EXPECTED_CAR_SIZE_IN_BYTES, "CAR size in bytes");

  assertEquals(response.url, requestUrl.toString());
});

test("can retrieve IPFS content using Fetch Request object", async () => {
  const request = new Request(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`);
  const response = await fetch(request);
  await assertResponseIsOk(response);

  const payload = await response.arrayBuffer();
  assertEquals(payload.byteLength, EXPECTED_CAR_SIZE_IN_BYTES, "CAR size in bytes");

  assertEquals(response.url, request.url);
});

test("does not modify original request headers - headers initialized as array", async () => {
  const request = new Request(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`, {
    headers: [["X-Test", "true"]],
  });
  const response = await fetch(request);
  await assertResponseIsOk(response);

  assertEquals(Array.from(request.headers.keys()), ["x-test"]);
});

test("does not modify original request headers - headers initialized as object", async () => {
  const request = new Request(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`, {
    headers: { "X-Test": "true" },
  });
  const response = await fetch(request);
  await assertResponseIsOk(response);

  assertEquals(Array.from(request.headers.keys()), ["x-test"]);
});

test("does not modify original request headers - headers initialized as Headers", async () => {
  const request = new Request(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`, {
    headers: new Headers({ "X-Test": "true" }),
  });
  const response = await fetch(request);
  await assertResponseIsOk(response);

  assertEquals(Array.from(request.headers.keys()), ["x-test"]);
});

test("rejects user-provided Authorization header", async () => {
  const request = new Request(`ipfs://${TEST_CID}${FRISBII_INSTANCE_QUERY_PARAMS}`, {
    headers: { Authorization: "invalid" },
  });

  let error = await assertRejects(() => fetch(request));
  assertMatch(error.message, /authorization/i);
});

/**
 * @param {Response} response Fetch API response
 */
async function assertResponseIsOk(response) {
  if (!response.ok) {
    throw new AssertionError(
      `Fetch request failed with status code ${response.status}: ${await response.text()}`,
    );
  }
}
