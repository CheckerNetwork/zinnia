// https://github.com/denoland/deno/blob/v2.2.8/cli/util/path.rs

// Copyright 2018-2025 the Deno authors. MIT license.

/// For decoding percent-encodeing string
/// could be used for module specifier string literal of local modules,
/// or local file path to display `non-ASCII` characters correctly
/// # Examples
/// ```plain
/// use crate::vendored::cli_util_path::to_percent_decoded_str;
///
/// let str = to_percent_decoded_str("file:///Users/path/to/%F0%9F%A6%95.ts");
/// assert_eq!(str, "file:///Users/path/to/🦕.ts");
/// ```
pub fn to_percent_decoded_str(s: &str) -> String {
    match percent_encoding::percent_decode_str(s).decode_utf8() {
        Ok(s) => s.to_string(),
        // when failed to decode, return the original string
        Err(_) => s.to_string(),
    }
}
