use std::time::Duration;

use roccat_vulcan_api_rs::{
    ColorBuffer, ColorRgb, ErrorRoccatVulcanApi, KeyName, KeyboardApi, Layout, LayoutFrCh,
    NUMBER_KEY_LED_BUFFER,
};

fn main() -> Result<(), ErrorRoccatVulcanApi> {
    let mut key_press_mask: [bool; NUMBER_KEY_LED_BUFFER] = [false; NUMBER_KEY_LED_BUFFER];
    let keyboard = KeyboardApi::new()?;
    let base_color = ColorRgb::new(0, 255, 255);
    let press_color = ColorRgb::new(255, 0, 255);
    let mut buffer = ColorBuffer::<ColorRgb>::from_element(base_color);
    let layout = LayoutFrCh::new();
    keyboard.render(&buffer)?;
    'mainloop: loop {
        let result = keyboard.read_key_press(Duration::from_millis(62));
        if let Ok(val) = result {
            for keypress in val {
                let a = layout.find_from_key_code(*keypress.key_code());
                if let Some(key) = a {
                    if *key.key_name() == KeyName::Escape {
                        break 'mainloop;
                    }
                    let index_key = key.key_code_light().code() as usize;
                    if index_key < buffer.buffer().len() {
                        buffer.buffer_mut()[index_key] = press_color;
                        key_press_mask[index_key] = keypress.is_pressed()
                    }
                }
            }
        }
        keyboard.render(&buffer)?;
        for (index, el) in buffer.buffer_mut().iter_mut().enumerate() {
            if *el != base_color && !key_press_mask[index] {
                if el.r() > base_color.r() {
                    *el.r_mut() -= 5.min(el.r() - base_color.r());
                } else {
                    *el.r_mut() += 5.min(base_color.r() - el.r());
                }
                if el.g() > base_color.g() {
                    *el.g_mut() -= 5.min(el.g() - base_color.g());
                } else {
                    *el.g_mut() += 5.min(base_color.g() - el.g());
                }
                if el.b() > base_color.b() {
                    *el.b_mut() -= 5.min(el.b() - base_color.b());
                } else {
                    *el.b_mut() += 5.min(base_color.b() - el.b());
                }
            }
        }
    }
    Ok(())
}
