use roccat_vulcan_api_rs::{
    ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyName, KeyboardApi, Layout, LayoutFrCh,
};

/// color the keyboard with cyan and change to color of the key pressed to blue
fn main() -> Result<(), ErrorRoccatVulcanApi> {
    let keyboard = KeyboardApi::new()?;
    let mut buffer = ColorBuffer::<ColorRgb>::from_element(ColorRgb::new(0, 255, 255));
    let layout = LayoutFrCh::new();

    loop {
        keyboard.render(&buffer)?;
        let result = keyboard.wait_for_key_press();
        if let Ok(val) = result {
            let a = layout.find_from_key_code(*val.key_code());
            if let Some(key) = a {
                let index_key = key.key_code_light().code() as usize;
                if index_key < buffer.buffer().len() {
                    if !val.is_pressed() {
                        if let KeyName::Escape = key.key_name() {
                            break;
                        }
                        buffer.buffer_mut()[index_key] = ColorRgb::new(0, 255, 255)
                    } else {
                        buffer.buffer_mut()[index_key] = ColorRgb::new(0, 0, 255)
                    }
                }
            }
        }
    }
    Ok(())
}
