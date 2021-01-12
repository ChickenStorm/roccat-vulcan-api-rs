
//! Provide an API to control the lighting of the Roccat Vulcan 100 and 120.
//! 
//! # Usage
//! 
//! add on your dependencies of Cargo.toml
//! `roccat_vulcan_api_rs = { version = "0.1.1", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs" }`.
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


pub use config::{
    constants,
    HidInterfaceInfo,
    HidInterfaceUserPageInfo,
    HidInterfaceFilter,
    KeyboardIntrefacesInfo,
    get_default_interface_info,
    get_products_id_default,
};
pub use keyboard::{
    KeyboardApi,
    ErrorRoccatVulcanApi
};
pub use layout::{
    layout_fr_ch::LayoutFrCh,
    Layout,
    Keypress,
    KeyCodePress,
    Key,
};
pub use color::{
    Color,
    ColorRgb,
    ColorRgba,
    ColorLuminosity,
    ColorBuffer
};

pub mod keyboard;
pub mod config;
pub mod layout;
pub mod color;

#[cfg(test)]
mod test;
