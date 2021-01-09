
//! Provide an API to control the lighting of the Roccat Vulcan 100 and 120.
//! 
//! # Usage
//! 
//! add on your dependencies of Cargo.toml
//! `roccat_vulcan_api_rs = { version = "0.1.0", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs" }`
//! # Examples
//! To load and initialised a keyboard use
//! ```
//! use roccat_vulcan_api_rs::{KeyboardApi, config, ControlerFeatureKind};
//! 
//! let mut keyboard = KeyboardApi::get_api.unwrap();
//! let result = keyboard.initialise_control_device(&ControlerFeatureKind::Rainbow);
//! ```

pub mod keyboard;
pub mod config;

pub use config::constants;
pub use keyboard::{
    KeyboardApi,
    ControlerFeatureKind,
    ErrorRoccatVulcanApi
};
pub use config::*;


pub mod color{
    
}

#[cfg(test)]
mod test;
