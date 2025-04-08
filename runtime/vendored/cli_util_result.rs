// https://github.com/denoland/deno/blob/v2.2.8/cli/lib/util/result.rs
// Copyright 2018-2025 the Deno authors. MIT license.

use std::fmt::Debug;
use std::fmt::Display;

use crate::deno_core::error::AnyError;
use crate::deno_core::error::CoreError;
use deno_error::JsErrorBox;
use deno_error::JsErrorClass;

pub fn any_and_jserrorbox_downcast_ref<E: Display + Debug + Send + Sync + 'static>(
    err: &AnyError,
) -> Option<&E> {
    err.downcast_ref::<E>()
        .or_else(|| {
            err.downcast_ref::<JsErrorBox>()
                .and_then(|e| e.as_any().downcast_ref::<E>())
        })
        .or_else(|| {
            err.downcast_ref::<CoreError>().and_then(|e| match e {
                CoreError::JsBox(e) => e.as_any().downcast_ref::<E>(),
                _ => None,
            })
        })
}
