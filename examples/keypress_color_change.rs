use roccat_vulcan_api_rs::layout::{layout_fr_ch::LayoutFrCh, Key, Layout};
use roccat_vulcan_api_rs::ColorBuffer;
use roccat_vulcan_api_rs::ColorRgb;
use roccat_vulcan_api_rs::KeyboardApi;

/// color the keybaord with cyan and change to color of the key pressed to blue
fn main() {
    let keyboard = KeyboardApi::get_api().unwrap();
    let mut buffer = ColorBuffer::<ColorRgb>::new(ColorRgb::new(0, 255, 255));
    let layout = LayoutFrCh::new();

    loop {
        keyboard.render(&buffer).unwrap();
        let result = keyboard.wait_for_key_press();
        if let Ok(val) = result {
            let a = layout.find_key_info_from_press_code(&val.key_code());
            if let Some(key) = a {
                let index_key = *key.key_code_light() as usize;
                if index_key < buffer.buffer().len() {
                    if !val.is_pressed() {
                        if let Key::Escape = key.key() {
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
}
