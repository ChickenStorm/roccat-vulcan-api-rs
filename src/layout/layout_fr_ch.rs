
use super::{
    KeyCodePress,
    Key,
    KeyInfo,
    Layout,
    KeyCodeLight,
};

/// get Layout key associative
fn get_layout_info_fr_ch() -> [KeyInfo; 107] {
    // I can not move that in a constant because of the .to_string
    [
        KeyInfo::new_from_key(3, KeyCodePress::new(10, 57), Key::CapsLock),
        KeyInfo::new_from_key(250, KeyCodePress::new(204, 1), Key::WheelUp),
        KeyInfo::new_from_key(251, KeyCodePress::new(204, 255), Key::WheelDown),
        KeyInfo::new_from_key(0, KeyCodePress::new(251, 17), Key::Escape),
        KeyInfo::new(1, KeyCodePress::new(251, 18), "§".to_string(), Key::Section),
        KeyInfo::new_from_key(2, KeyCodePress::new(251, 20), Key::Tab),
        KeyInfo::new_from_key(4, KeyCodePress::new(251, 22), Key::LeftShift),
        KeyInfo::new_from_key(5, KeyCodePress::new(251, 23), Key::LeftControl),
        KeyInfo::new(6, KeyCodePress::new(251, 19), "1".to_string(), Key::Key1),
        KeyInfo::new_from_key(7, KeyCodePress::new(251, 26), Key::Q),
        KeyInfo::new_from_key(8, KeyCodePress::new(251, 28), Key::A),
        KeyInfo::new(9, KeyCodePress::new(251, 29), "<".to_string(), Key::LessThan),
        KeyInfo::new_from_key(10, KeyCodePress::new(251, 31), Key::Super),

        KeyInfo::new_from_key(11, KeyCodePress::new(251, 16), Key::F1),
        KeyInfo::new(12, KeyCodePress::new(251, 25), "2".to_string(), Key::Key1),
        KeyInfo::new_from_key(13, KeyCodePress::new(251, 27), Key::W),
        KeyInfo::new_from_key(14, KeyCodePress::new(251, 37), Key::S),
        KeyInfo::new_from_key(15, KeyCodePress::new(251, 38), Key::Y),
        KeyInfo::new_from_key(16, KeyCodePress::new(251, 39), Key::Alt),
        
        KeyInfo::new_from_key(17, KeyCodePress::new(251, 24), Key::F2),
        KeyInfo::new(18, KeyCodePress::new(251, 34), "3".to_string(), Key::Key3),
        KeyInfo::new_from_key(19, KeyCodePress::new(251, 36), Key::E),
        KeyInfo::new_from_key(20, KeyCodePress::new(251, 44), Key::D),
        KeyInfo::new_from_key(21, KeyCodePress::new(251, 45), Key::X),
        
        KeyInfo::new_from_key(23, KeyCodePress::new(251, 33), Key::F3),
        KeyInfo::new(24, KeyCodePress::new(251, 35), "4".to_string(), Key::Key4),
        KeyInfo::new_from_key(25, KeyCodePress::new(251, 43), Key::R),
        KeyInfo::new_from_key(26, KeyCodePress::new(251, 53), Key::F),
        KeyInfo::new_from_key(27, KeyCodePress::new(251, 46), Key::C),
        
        KeyInfo::new_from_key(28, KeyCodePress::new(251, 32), Key::F4),
        KeyInfo::new(29, KeyCodePress::new(251, 42), "5".to_string(), Key::Key5),
        KeyInfo::new_from_key(30, KeyCodePress::new(251, 51), Key::T),
        KeyInfo::new_from_key(31, KeyCodePress::new(251, 52), Key::G),
        KeyInfo::new_from_key(32, KeyCodePress::new(251, 54), Key::V),
        
        KeyInfo::new(33, KeyCodePress::new(251, 41), "6".to_string(), Key::Key6),
        KeyInfo::new_from_key(34, KeyCodePress::new(251, 59), Key::Z),
        KeyInfo::new_from_key(35, KeyCodePress::new(251, 61), Key::H),
        KeyInfo::new_from_key(36, KeyCodePress::new(251, 62), Key::B),
        KeyInfo::new_from_key(37, KeyCodePress::new(251, 63), Key::Space),
        
        KeyInfo::new_from_key(48, KeyCodePress::new(251, 40), Key::F5),
        KeyInfo::new(49, KeyCodePress::new(251, 49), "7".to_string(), Key::Key7),
        KeyInfo::new_from_key(50, KeyCodePress::new(251, 60), Key::U),
        KeyInfo::new_from_key(51, KeyCodePress::new(251, 68), Key::J),
        KeyInfo::new_from_key(52, KeyCodePress::new(251, 71), Key::N),
        
        KeyInfo::new_from_key(53, KeyCodePress::new(251, 48), Key::F6),
        KeyInfo::new(54, KeyCodePress::new(251, 66), "8".to_string(), Key::Key8),
        KeyInfo::new_from_key(55, KeyCodePress::new(251, 67), Key::I),
        KeyInfo::new_from_key(56, KeyCodePress::new(251, 69), Key::K),
        KeyInfo::new_from_key(57, KeyCodePress::new(251, 70), Key::M),
        
        KeyInfo::new_from_key(59, KeyCodePress::new(251, 56), Key::F7),
        KeyInfo::new(60, KeyCodePress::new(251, 65), "9".to_string(), Key::Key9),
        KeyInfo::new_from_key(61, KeyCodePress::new(251, 76), Key::O),
        KeyInfo::new_from_key(62, KeyCodePress::new(251, 77), Key::L),
        KeyInfo::new(63, KeyCodePress::new(251, 78), ",".to_string(), Key::Comma),
        
        KeyInfo::new_from_key(65, KeyCodePress::new(251, 57), Key::F8),
        KeyInfo::new(66, KeyCodePress::new(251, 74), "0".to_string(), Key::Key0),
        KeyInfo::new_from_key(67, KeyCodePress::new(251, 84), Key::P),
        KeyInfo::new(68, KeyCodePress::new(251, 85), "é".to_string(), Key::EAcute),
        KeyInfo::new(69, KeyCodePress::new(251, 86), ".".to_string(), Key::Dot),
        KeyInfo::new_from_key(70, KeyCodePress::new(251, 103), Key::AltGr),
        
        KeyInfo::new(72, KeyCodePress::new(251, 75), "'".to_string(), Key::Apostrophe),
        KeyInfo::new(73, KeyCodePress::new(251, 91), "è".to_string(), Key::EGrave),
        KeyInfo::new(74, KeyCodePress::new(251, 93), "à".to_string(), Key::AGrave),
        KeyInfo::new(75, KeyCodePress::new(251, 94), "-".to_string(), Key::Dash),
        KeyInfo::new_from_key(76, KeyCodePress::new(251, 119), Key::Function),
        
        KeyInfo::new_from_key(78, KeyCodePress::new(251, 64), Key::F9),
        KeyInfo::new(79, KeyCodePress::new(251, 83), "^".to_string(), Key::Caret),
        KeyInfo::new(80, KeyCodePress::new(251, 92), "¨".to_string(), Key::Diaeresis),
        KeyInfo::new_from_key(82, KeyCodePress::new(251, 110), Key::RightShift),
        KeyInfo::new_from_key(83, KeyCodePress::new(251, 127), Key::Menu),
        
        KeyInfo::new_from_key(84, KeyCodePress::new(251, 72), Key::F10),
        KeyInfo::new_from_key(85, KeyCodePress::new(251, 80), Key::F11),
        KeyInfo::new_from_key(86, KeyCodePress::new(251, 81), Key::F12),
        KeyInfo::new_from_key(87, KeyCodePress::new(251, 73), Key::BackSpace),
        KeyInfo::new_from_key(88, KeyCodePress::new(251, 107), Key::Enter),
        KeyInfo::new_from_key(89, KeyCodePress::new(251, 135), Key::RightControl),
        
        KeyInfo::new(96, KeyCodePress::new(251, 100), "$".to_string(), Key::Dolar),
        KeyInfo::new_from_key(99, KeyCodePress::new(251, 88), Key::PrintScreen),
        KeyInfo::new_from_key(100, KeyCodePress::new(251, 89), Key::Insert),
        KeyInfo::new_from_key(101, KeyCodePress::new(251, 90), Key::Delete),
        KeyInfo::new_from_key(102, KeyCodePress::new(251, 109), Key::ArrowLeft),
        
        KeyInfo::new_from_key(103, KeyCodePress::new(251, 96), Key::ScrollLock),
        KeyInfo::new_from_key(104, KeyCodePress::new(251, 97), Key::Home),
        KeyInfo::new_from_key(105, KeyCodePress::new(251, 98), Key::End),
        KeyInfo::new_from_key(106, KeyCodePress::new(251, 108), Key::ArrowUp),
        KeyInfo::new_from_key(107, KeyCodePress::new(251, 117), Key::ArrowDown),
        
        KeyInfo::new_from_key(108, KeyCodePress::new(251, 104), Key::Break),
        KeyInfo::new_from_key(109, KeyCodePress::new(251, 105), Key::PageUp),
        KeyInfo::new_from_key(110, KeyCodePress::new(251, 106), Key::PageDown),
        KeyInfo::new_from_key(111, KeyCodePress::new(251, 125), Key::ArrowRight),
        
        KeyInfo::new_from_key(113, KeyCodePress::new(251, 113), Key::NumLock),
        KeyInfo::new_from_key(114, KeyCodePress::new(251, 114), Key::Num7),
        KeyInfo::new_from_key(115, KeyCodePress::new(251, 115), Key::Num4),
        KeyInfo::new_from_key(116, KeyCodePress::new(251, 116), Key::Num1),
        KeyInfo::new_from_key(117, KeyCodePress::new(251, 133), Key::Num0),
        
        KeyInfo::new_from_key(119, KeyCodePress::new(251, 121), Key::NumDivide),
        KeyInfo::new_from_key(120, KeyCodePress::new(251, 122), Key::Num8),
        KeyInfo::new_from_key(121, KeyCodePress::new(251, 123), Key::Num5),
        KeyInfo::new_from_key(122, KeyCodePress::new(251, 124), Key::Num2),
        
        KeyInfo::new_from_key(124, KeyCodePress::new(251, 129), Key::NumMultiply),
        KeyInfo::new_from_key(125, KeyCodePress::new(251, 130), Key::Num9),
        KeyInfo::new_from_key(126, KeyCodePress::new(251, 131), Key::Num6),
        KeyInfo::new_from_key(127, KeyCodePress::new(251, 132), Key::Num3),
        KeyInfo::new_from_key(128, KeyCodePress::new(251, 141), Key::NumDot),
        
        KeyInfo::new_from_key(129, KeyCodePress::new(251, 137), Key::NumMinus),
        KeyInfo::new_from_key(130, KeyCodePress::new(251, 138), Key::NumPlus),
        KeyInfo::new_from_key(131, KeyCodePress::new(251, 140), Key::NumEnter),
    ]
}

// Swiss French Layout
pub struct LayoutFrCh {
    layout_key_info : [KeyInfo; 107]
}

impl LayoutFrCh {
    pub fn new() -> Self {
        Self {
            layout_key_info : get_layout_info_fr_ch()
        }
    }
    
    pub fn layout_key_info (&self) -> &[KeyInfo; 107] {
        return &self.layout_key_info;
    }
    
}

impl Layout for LayoutFrCh {
    fn find_key_info_form_light(&self, key_code_light : &KeyCodeLight) -> Option<&KeyInfo> {
        return self.layout_key_info.iter()
            .filter(move |element| *key_code_light == *element.key_code_light())
            .next();
    }
    
    fn find_key_info_form_press_code(&self, key_code_press : &KeyCodePress) -> Option<&KeyInfo> {
        return self.layout_key_info.iter()
            .filter(move |element| *key_code_press == *element.key_code_press())
            .next();
    }
    
    fn find_key_info_form_key(&self, key : &Key) -> Option<&KeyInfo> {
        return self.layout_key_info.iter()
            .filter(move |element| *key == *element.key())
            .next();
    }
    
    fn find_key_info_form_string(&self, string : &String) -> Option<&KeyInfo> {
        return self.layout_key_info.iter()
            .filter(move |element| *string == *element.key_string())
            .next();
    }
}
