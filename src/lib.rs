//! Native Delta Lake snapshot planning over the Polars host storage interface.
use std::panic::AssertUnwindSafe;

use polars_io_ext_ffi::common::{FfiError, FfiErrorKind};
pub mod delta;
#[cfg(feature = "python")]
mod python;
pub const PACKAGE_NAME: &str = "polars_delta";
pub(crate) fn catch_panic<T>(f: impl FnOnce() -> Result<T, FfiError>) -> Result<T, FfiError> {
    std::panic::catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| {
        Err(FfiError::new(
            FfiErrorKind::PANIC,
            "polars_delta planner panicked",
        ))
    })
}
