use std::path::Path;
use std::rc::Rc;

use deno_core::anyhow::Result;
use deno_core::url::Url;
use deno_core::{include_js_files, Extension};
use deno_fetch::FetchPermissions;
use deno_web::TimersPermission;

use crate::Reporter;

pub struct Options {
    pub reporter: Rc<dyn Reporter>,
}

/// Hard-coded permissions
pub struct ZinniaPermissions;

impl TimersPermission for ZinniaPermissions {
    fn allow_hrtime(&mut self) -> bool {
        // Disable high-resolution time management.
        //
        // Quoting from https://v8.dev/docs/untrusted-code-mitigations
        // > A high-precision timer makes it easier to observe side channels in the SSCA
        // > vulnerability. If your product offers high-precision timers that can be accessed by
        // > untrusted JavaScript or WebAssembly code, consider making these timers more coarse or
        // > adding jitter to them.
        false
    }
    fn check_unstable(&self, _state: &deno_core::OpState, _api_name: &'static str) {}
}

impl FetchPermissions for ZinniaPermissions {
    fn check_net_url(&mut self, _url: &Url, _api_name: &str) -> Result<()> {
        Ok(())
    }
    fn check_read(&mut self, _p: &Path, _api_name: &str) -> Result<()> {
        Ok(())
    }
}

pub fn init(options: Options) -> Extension {
    Extension::builder("zinnia_runtime")
        .esm(include_js_files!(
          dir "js",
          "06_util.js",
          "90_zinnia_apis.js",
          "98_global_scope.js",
          "99_main.js",
        ))
        .state(move |state| {
            state.put(ZinniaPermissions {});
            state.put(Rc::clone(&options.reporter));
        })
        .build()
}
