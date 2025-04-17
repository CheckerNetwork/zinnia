// This TypeScript-only block of code is removed during transpilation. A naive solution that removes
// the TypeScript code instead of replacing it with whitespace and does not apply source maps to error
// stack traces will lead to incorrect line numbers in error stack traces.
interface User {
  name: string;
  email: string;
}

// The part `: Error` changes the source column number
// between the TypeScript original and the transpiled code.
//
// Throw the error so that the test can verify source code line & column numbers
// in the stack trace frames but also the line throwing the exception.
const error: Error = new Error(); throw error;
