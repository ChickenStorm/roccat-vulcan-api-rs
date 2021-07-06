//! Provide an API to control the lighting of the Roccat Vulcan 100 and 120.
//!
//! # Usage
//!
//! add on your dependencies of Cargo.toml
//! `roccat-vulcan-api-rs = { version = "0.1.1", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs", branch = "main" }`.
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
//! use roccat_vulcan_api_rs::{KeyboardApi, config, ColorBuffer, ColorRgb};
//! use std::{
//!     thread::sleep,
//!     time::Duration,
//! };
//!
//! let keyboard = KeyboardApi::get_api().unwrap();
//! let buffer = ColorBuffer::new(ColorRgb::new(255, 255, 255));
//! keyboard.render(&buffer).unwrap();
//! sleep(Duration::from_secs(1));
//! ```

pub use color::{Color, ColorBuffer, ColorLuminosity, ColorRgb, ColorRgba};
pub use config::{
    constants, get_default_interface_info, get_products_id_default, HidInterfaceFilter,
    HidInterfaceInfo, HidInterfaceUserPageInfo, KeyboardIntrefacesInfo,
};
pub use keyboard::{ErrorRoccatVulcanApi, KeyboardApi};
pub use layout::{layout_fr_ch::LayoutFrCh, Key, KeyCodePress, Keypress, Layout};

pub mod color;
pub mod config;
pub mod keyboard;
pub mod layout;

#[cfg(test)]
mod test;
