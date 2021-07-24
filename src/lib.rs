//! Provide an API to control the lighting of the Roccat Vulcan 100 and 120.
//!
//! # Usage
//!
//! add on your dependencies of Cargo.toml
//! `roccat-vulcan-api-rs = { version = "0.2.1", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs", branch = "main" }`.
//!
//! The main way to interact with the API is through [`KeyboardApi`].
//! Note that when the structure is dropped the keyboard will go back to the default rainbow behavior.
//!
//!
//! # Layout
//! For the moment only Swiss French layout is supported. To support other layout implement the trait [`Layout`].
//!
//! # Examples
//! To load and initialized a keyboard use
//! ```
//! use std::{thread::sleep, time::Duration};
//!
//! use roccat_vulcan_api_rs::{ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyboardApi};
//!
//! # fn main() -> Result<(), ErrorRoccatVulcanApi> {
//! # #[cfg(not(feature = "no-keyboard-test"))]
//! # {
//! let keyboard = KeyboardApi::new()?;
//! let buffer = ColorBuffer::from_element(ColorRgb::new(255, 255, 255));
//! keyboard.render(&buffer)?;
//! sleep(Duration::from_secs(1));
//! # }
//! # Ok(())
//! # }
//! ```

// TODO
// - doc
// - unit test
// - Improve key position
// - Englobing obj (buffer and layout coordination)
// - easier use of library
// - multi threading
// - API Check liste
// - more color otions

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
#![doc(html_root_url = "https://docs.rs/roccat-vulcan-api-rs/0.2.1")]
#![warn(clippy::all)]
#![warn(clippy::exhaustive_enums)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_docs_in_private_items)]
//#![doc(test(attr(deny(warnings))))]

#[doc(inline)]
mod color;
#[doc(inline)]
mod error;
#[doc(inline)]
mod interface;
#[doc(inline)]
mod keyboard;
#[doc(inline)]
mod layout;
#[doc(inline)]
mod reports;

pub use color::*;
pub use error::*;
pub use interface::*;
pub use keyboard::*;
pub use layout::*;

#[cfg(test)]
mod test;
