// Signal that we are going to start the retrieval
Zinnia.activity.info("fetch:start");
const response = await fetch(
  "ipfs://bafybeih5zasorm4tlfga4ztwvm2dlnw6jxwwuvgnokyt3mjamfn3svvpyy?protocol=http&providers=/dns4/frisbii.fly.dev/https",
);

Zinnia.activity.info("fetch:response-headers");

// Introduce some delay before reading the content
await new Promise((resolve) => setTimeout(resolve, 5000));

// Read the content
for await (const _chunk of response.body) {
  // drop the chunk
}

// Signal that the retrieval has finished. If this happens then the test
// did not kill the zinniad process quickly enough.
Zinnia.activity.info("fetch:end");
