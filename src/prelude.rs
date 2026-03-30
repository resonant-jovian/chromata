//! Convenience re-exports for common chromata types.
//!
//! ```rust
//! use chromata::prelude::*;
//! ```

pub use crate::{Color, Contrast, Theme, ThemeBuilder, Variant};
pub use crate::{collect_all_themes, filter_by_contrast, filter_by_variant, find_by_name};
pub use alloc::borrow::Cow;
