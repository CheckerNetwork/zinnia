// global `self`
if (typeof self !== "object") {
  throw new Error(`Expected \`self\` to have type "object" but found "${typeof self}"`);
}

if (self != globalThis) {
  throw new Error("Expected `self` to be `globalThis`");
}
