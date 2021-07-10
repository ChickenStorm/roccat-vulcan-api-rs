use roccat_vulcan_api_rs::{
    color::{Color, ColorBuffer, ColorRgb},
    constants,
    keyboard::KeyboardApi,
    layout::{layout_fr_ch::LayoutFrCh, Key, Layout},
};
use std::time::Duration;

fn main() {
    let api = hidapi::HidApi::new().unwrap();

    let mut key_press_mask: [bool; constants::NUMBER_KEY_LED_BUFFER] =
        [false; constants::NUMBER_KEY_LED_BUFFER];
    let keyboard = KeyboardApi::get_api_from_hidapi(&api).unwrap();
    let base_color = ColorRgb::new(255, 255, 255);
    let press_color = vec![
        ColorRgb::new(255, 0, 255),
        ColorRgb::new(0, 0, 255),
        ColorRgb::new(255, 0, 0),
        ColorRgb::new(0, 255, 255),
    ];
    let mut press_count: usize = 0;
    let mut buffer = ColorBuffer::<ColorRgb>::new(base_color);
    let layout = LayoutFrCh::new();
    keyboard.render(&buffer).unwrap();
    'mainloop: loop {
        let result = keyboard.read_key_press(Duration::from_millis(62));
        if let Ok(val) = result {
            for keypress in val {
                let a = layout.find_key_info_from_press_code(&keypress.key_code());
                if let Some(key) = a {
                    if *key.key() == Key::Escape {
                        break 'mainloop;
                    }
                    let index_key = *key.key_code_light() as usize;
                    if index_key < buffer.buffer().len() {
                        if keypress.is_pressed() {
                            buffer.buffer_mut()[index_key] =
                                press_color[press_count % press_color.len()];
                            press_count += 1;
                        }
                        key_press_mask[index_key] = keypress.is_pressed();
                    }
                }
            }
        }
        keyboard.render(&buffer).unwrap();
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
}
