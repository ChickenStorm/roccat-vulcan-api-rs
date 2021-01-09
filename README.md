# roccat-vulcan-api-rs
**Roccat Vulcan keyboard illumination API.**  
A fast multiplatform API to control Roccat Vulcan illumination.

## Usage
add on your dependencies of Cargo.toml
`roccat_vulcan_api_rs = { version = "0.1.0", git = "https://github.com/ChickenStorm/roccat-vulcan-api-rs" }`
# Examples
To load and initialised a keyboard use
```rust
use roccat_vulcan_api_rs::{KeyboardApi, config, ControlerFeatureKind};

let mut keyboard = KeyboardApi::get_api.unwrap();
let result = keyboard.initialise_control_device(&ControlerFeatureKind::Rainbow);
```



### Acknowledgement

- I thank [simonhuwiler](https://github.com/simonhuwiler/roccatvulcan) and [duncanthrax](https://github.com/duncanthrax/roccat-vulcan) for their code which this library is based on.  
- And a big thank you to Bidoubioche for helping me learn rust and for always answering my question and the help figuring my way through the compiler messages 
