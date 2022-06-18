#![doc = include_str!("../README.md")]
// TODO
// - doc
// - unit test
// - Improve key position
// - Englobing obj (buffer and layout coordination)
// - easier use of library
// - multi threading
// - API Check list
// - more color options
// - display / debug keyboard api by storing hdiapi::DeviceInfo
// - builder

//#![warn(clippy::as_conversions)]
#![warn(clippy::cast_sign_loss)]
#![warn(clippy::cast_possible_truncation)]
#![warn(clippy::cast_possible_wrap)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::cognitive_complexity)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::implicit_hasher)]
#![warn(clippy::implicit_saturating_sub)]
#![warn(clippy::imprecise_flops)]
#![warn(clippy::large_types_passed_by_value)]
#![warn(clippy::macro_use_imports)]
#![warn(clippy::manual_ok_or)]
#![warn(clippy::missing_const_for_fn)]
#![warn(clippy::needless_pass_by_value)]
#![warn(clippy::non_ascii_literal)]
#![warn(clippy::semicolon_if_nothing_returned)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::todo)]
#![warn(clippy::trivially_copy_pass_by_ref)]
#![warn(clippy::type_repetition_in_bounds)]
#![warn(clippy::unreadable_literal)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_self)]
#![warn(clippy::unnecessary_wraps)]
#![warn(clippy::missing_errors_doc)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/roccat-vulcan-api-rs/0.2.3")]
#![warn(clippy::all)]
#![warn(clippy::exhaustive_enums)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//#![doc(test(attr(deny(warnings))))]

mod color;
mod error;
mod interface;
mod keyboard;
mod layout;
mod reports;

#[cfg(test)]
mod test;

#[doc(inline)]
pub use color::*;
#[doc(inline)]
pub use error::*;
#[doc(inline)]
pub use interface::*;
#[doc(inline)]
pub use keyboard::*;
#[doc(inline)]
pub use layout::*;
