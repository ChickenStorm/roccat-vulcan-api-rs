# roccat-vulcan-api-rs
**Roccat Vulcan keyboard illumination API.**  
A fast multiplatform API to control Roccat Vulcan illumination.

Provide an API to control the lighting of the Roccat Vulcan 100 and 120.

# Usage

add on your dependencies of Cargo.toml
`roccat-vulcan-api-rs = { version = "0.1.1", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs", branch = "main" }`.


The main way to interact with the API is through `KeyboardApi`.
Note that when the structure is dropped the keyboard will go back to the default rainbow behavior.


# Layout
For the moment only Swiss French layout is supported. To support other layout implement the trait `Layout`.

# Examples
To load and initialized a keyboard use
```rust
use roccat_vulcan_api_rs::{KeyboardApi, config, ColorBuffer, ColorRgb};
use std::{
    thread::sleep,
    time::Duration,
};

let keyboard = KeyboardApi::get_api().unwrap();
let buffer = ColorBuffer::new(ColorRgb::new(255, 255, 255));
keyboard.render(&buffer).unwrap();
sleep(Duration::from_secs(1));
```

### Acknowledgement

- I thank [simonhuwiler](https://github.com/simonhuwiler/roccatvulcan) and [duncanthrax](https://github.com/duncanthrax/roccat-vulcan) for their code which this library is based on.  
- And a big thank you to Bidoubioche for helping me learn rust and for always answering my question and the help figuring my way through the compiler messages 
