

# roccat-vulcan-api-rs

![license](https://img.shields.io/github/license/ChickenStorm/roccat-vulcan-api-rs)
[![](https://img.shields.io/badge/doc-Read_Me-blue)](https://chickenstorm.github.io/roccat-vulcan-api-rs/roccat_vulcan_api_rs/index.html)
![Build](https://img.shields.io/github/workflow/status/ChickenStorm/roccat-vulcan-api-rs/Rust)
[![codecov](https://codecov.io/gh/ChickenStorm/roccat-vulcan-api-rs/branch/main/graph/badge.svg?token=Z1J8KMKLC1)](https://codecov.io/gh/ChickenStorm/roccat-vulcan-api-rs)

**Roccat Vulcan keyboard illumination API.**  
A fast multiplatform API to control Roccat Vulcan illumination.

Provide an API to control the lighting of the Roccat Vulcan 100 and 120.

# Usage

add on your dependencies of Cargo.toml
`roccat-vulcan-api-rs = { version = "0.2.2", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs", branch = "main" }`.

The main way to interact with the API is through [`KeyboardApi`].
Note that when the structure is dropped the keyboard will go back to the default rainbow behavior.


# Layout
For the moment only Swiss French layout is supported. To support other layout implement the trait [`Layout`].

# Examples
To load and initialized a keyboard use
```rust
use std::{thread::sleep, time::Duration};
use roccat_vulcan_api_rs::{ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyboardApi};

# fn main() -> Result<(), ErrorRoccatVulcanApi> {
# #[cfg(not(feature = "no-keyboard-test"))]
# {
let keyboard = KeyboardApi::new()?;
let buffer = ColorBuffer::from_element(ColorRgb::new(255, 255, 255));
keyboard.render(&buffer)?;
sleep(Duration::from_secs(1));
# }
# Ok(())
# }
```
