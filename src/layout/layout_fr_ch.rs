//! Defines the layout for Swiss French layout

use super::{KeyCode, KeyInfo, KeyLight, KeyName, Layout};
#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

/// get Layout key associative
const fn get_layout_info_fr_ch() -> [KeyInfo; 107] {
    // I can not move that in a constant because of the .to_string
    // Latter I might change the key info to have a &'static str
    [
        KeyInfo::new(KeyLight::new(3), KeyCode::new(10, 57), KeyName::CapsLock),
        // note that the index for the key light i out of bound for the wheel up and down
        KeyInfo::new(KeyLight::new(250), KeyCode::new(204, 1), KeyName::WheelUp),
        KeyInfo::new(
            KeyLight::new(251),
            KeyCode::new(204, 255),
            KeyName::WheelDown,
        ),
        KeyInfo::new(KeyLight::new(0), KeyCode::new(251, 17), KeyName::Escape),
        KeyInfo::new(KeyLight::new(1), KeyCode::new(251, 18), KeyName::Section),
        KeyInfo::new(KeyLight::new(2), KeyCode::new(251, 20), KeyName::Tab),
        KeyInfo::new(KeyLight::new(4), KeyCode::new(251, 22), KeyName::LeftShift),
        KeyInfo::new(
            KeyLight::new(5),
            KeyCode::new(251, 23),
            KeyName::LeftControl,
        ),
        KeyInfo::new(KeyLight::new(6), KeyCode::new(251, 19), KeyName::Key1),
        KeyInfo::new(KeyLight::new(7), KeyCode::new(251, 26), KeyName::Q),
        KeyInfo::new(KeyLight::new(8), KeyCode::new(251, 28), KeyName::A),
        KeyInfo::new(KeyLight::new(9), KeyCode::new(251, 29), KeyName::LessThan),
        KeyInfo::new(KeyLight::new(10), KeyCode::new(251, 31), KeyName::Super),
        KeyInfo::new(KeyLight::new(11), KeyCode::new(251, 16), KeyName::F1),
        KeyInfo::new(KeyLight::new(12), KeyCode::new(251, 25), KeyName::Key2),
        KeyInfo::new(KeyLight::new(13), KeyCode::new(251, 27), KeyName::W),
        KeyInfo::new(KeyLight::new(14), KeyCode::new(251, 37), KeyName::S),
        KeyInfo::new(KeyLight::new(15), KeyCode::new(251, 38), KeyName::Y),
        KeyInfo::new(KeyLight::new(16), KeyCode::new(251, 39), KeyName::Alt),
        KeyInfo::new(KeyLight::new(17), KeyCode::new(251, 24), KeyName::F2),
        KeyInfo::new(KeyLight::new(18), KeyCode::new(251, 34), KeyName::Key3),
        KeyInfo::new(KeyLight::new(19), KeyCode::new(251, 36), KeyName::E),
        KeyInfo::new(KeyLight::new(20), KeyCode::new(251, 44), KeyName::D),
        KeyInfo::new(KeyLight::new(21), KeyCode::new(251, 45), KeyName::X),
        KeyInfo::new(KeyLight::new(23), KeyCode::new(251, 33), KeyName::F3),
        KeyInfo::new(KeyLight::new(24), KeyCode::new(251, 35), KeyName::Key4),
        KeyInfo::new(KeyLight::new(25), KeyCode::new(251, 43), KeyName::R),
        KeyInfo::new(KeyLight::new(26), KeyCode::new(251, 53), KeyName::F),
        KeyInfo::new(KeyLight::new(27), KeyCode::new(251, 46), KeyName::C),
        KeyInfo::new(KeyLight::new(28), KeyCode::new(251, 32), KeyName::F4),
        KeyInfo::new(KeyLight::new(29), KeyCode::new(251, 42), KeyName::Key5),
        KeyInfo::new(KeyLight::new(30), KeyCode::new(251, 51), KeyName::T),
        KeyInfo::new(KeyLight::new(31), KeyCode::new(251, 52), KeyName::G),
        KeyInfo::new(KeyLight::new(32), KeyCode::new(251, 54), KeyName::V),
        KeyInfo::new(KeyLight::new(33), KeyCode::new(251, 41), KeyName::Key6),
        KeyInfo::new(KeyLight::new(34), KeyCode::new(251, 59), KeyName::Z),
        KeyInfo::new(KeyLight::new(35), KeyCode::new(251, 61), KeyName::H),
        KeyInfo::new(KeyLight::new(36), KeyCode::new(251, 62), KeyName::B),
        KeyInfo::new(KeyLight::new(37), KeyCode::new(251, 63), KeyName::Space),
        KeyInfo::new(KeyLight::new(48), KeyCode::new(251, 40), KeyName::F5),
        KeyInfo::new(KeyLight::new(49), KeyCode::new(251, 49), KeyName::Key7),
        KeyInfo::new(KeyLight::new(50), KeyCode::new(251, 60), KeyName::U),
        KeyInfo::new(KeyLight::new(51), KeyCode::new(251, 68), KeyName::J),
        KeyInfo::new(KeyLight::new(52), KeyCode::new(251, 71), KeyName::N),
        KeyInfo::new(KeyLight::new(53), KeyCode::new(251, 48), KeyName::F6),
        KeyInfo::new(KeyLight::new(54), KeyCode::new(251, 66), KeyName::Key8),
        KeyInfo::new(KeyLight::new(55), KeyCode::new(251, 67), KeyName::I),
        KeyInfo::new(KeyLight::new(56), KeyCode::new(251, 69), KeyName::K),
        KeyInfo::new(KeyLight::new(57), KeyCode::new(251, 70), KeyName::M),
        KeyInfo::new(KeyLight::new(59), KeyCode::new(251, 56), KeyName::F7),
        KeyInfo::new(KeyLight::new(60), KeyCode::new(251, 65), KeyName::Key9),
        KeyInfo::new(KeyLight::new(61), KeyCode::new(251, 76), KeyName::O),
        KeyInfo::new(KeyLight::new(62), KeyCode::new(251, 77), KeyName::L),
        KeyInfo::new(KeyLight::new(63), KeyCode::new(251, 78), KeyName::Comma),
        KeyInfo::new(KeyLight::new(65), KeyCode::new(251, 57), KeyName::F8),
        KeyInfo::new(KeyLight::new(66), KeyCode::new(251, 74), KeyName::Key0),
        KeyInfo::new(KeyLight::new(67), KeyCode::new(251, 84), KeyName::P),
        KeyInfo::new(KeyLight::new(68), KeyCode::new(251, 85), KeyName::EAcute),
        KeyInfo::new(KeyLight::new(69), KeyCode::new(251, 86), KeyName::Dot),
        KeyInfo::new(KeyLight::new(70), KeyCode::new(251, 103), KeyName::AltGr),
        KeyInfo::new(
            KeyLight::new(72),
            KeyCode::new(251, 75),
            KeyName::Apostrophe,
        ),
        KeyInfo::new(KeyLight::new(73), KeyCode::new(251, 91), KeyName::EGrave),
        KeyInfo::new(KeyLight::new(74), KeyCode::new(251, 93), KeyName::AGrave),
        KeyInfo::new(KeyLight::new(75), KeyCode::new(251, 94), KeyName::Dash),
        KeyInfo::new(KeyLight::new(76), KeyCode::new(251, 119), KeyName::Function),
        KeyInfo::new(KeyLight::new(78), KeyCode::new(251, 64), KeyName::F9),
        KeyInfo::new(KeyLight::new(79), KeyCode::new(251, 83), KeyName::Caret),
        KeyInfo::new(KeyLight::new(80), KeyCode::new(251, 92), KeyName::Diaeresis),
        KeyInfo::new(
            KeyLight::new(82),
            KeyCode::new(251, 110),
            KeyName::RightShift,
        ),
        KeyInfo::new(KeyLight::new(83), KeyCode::new(251, 127), KeyName::Menu),
        KeyInfo::new(KeyLight::new(84), KeyCode::new(251, 72), KeyName::F10),
        KeyInfo::new(KeyLight::new(85), KeyCode::new(251, 80), KeyName::F11),
        KeyInfo::new(KeyLight::new(86), KeyCode::new(251, 81), KeyName::F12),
        KeyInfo::new(KeyLight::new(87), KeyCode::new(251, 73), KeyName::BackSpace),
        KeyInfo::new(KeyLight::new(88), KeyCode::new(251, 107), KeyName::Enter),
        KeyInfo::new(
            KeyLight::new(89),
            KeyCode::new(251, 135),
            KeyName::RightControl,
        ),
        KeyInfo::new(KeyLight::new(96), KeyCode::new(251, 100), KeyName::Dolar),
        KeyInfo::new(
            KeyLight::new(99),
            KeyCode::new(251, 88),
            KeyName::PrintScreen,
        ),
        KeyInfo::new(KeyLight::new(100), KeyCode::new(251, 89), KeyName::Insert),
        KeyInfo::new(KeyLight::new(101), KeyCode::new(251, 90), KeyName::Delete),
        KeyInfo::new(
            KeyLight::new(102),
            KeyCode::new(251, 109),
            KeyName::ArrowLeft,
        ),
        KeyInfo::new(
            KeyLight::new(103),
            KeyCode::new(251, 96),
            KeyName::ScrollLock,
        ),
        KeyInfo::new(KeyLight::new(104), KeyCode::new(251, 97), KeyName::Home),
        KeyInfo::new(KeyLight::new(105), KeyCode::new(251, 98), KeyName::End),
        KeyInfo::new(KeyLight::new(106), KeyCode::new(251, 108), KeyName::ArrowUp),
        KeyInfo::new(
            KeyLight::new(107),
            KeyCode::new(251, 117),
            KeyName::ArrowDown,
        ),
        KeyInfo::new(KeyLight::new(108), KeyCode::new(251, 104), KeyName::Break),
        KeyInfo::new(KeyLight::new(109), KeyCode::new(251, 105), KeyName::PageUp),
        KeyInfo::new(
            KeyLight::new(110),
            KeyCode::new(251, 106),
            KeyName::PageDown,
        ),
        KeyInfo::new(
            KeyLight::new(111),
            KeyCode::new(251, 125),
            KeyName::ArrowRight,
        ),
        KeyInfo::new(KeyLight::new(113), KeyCode::new(251, 113), KeyName::NumLock),
        KeyInfo::new(KeyLight::new(114), KeyCode::new(251, 114), KeyName::Num7),
        KeyInfo::new(KeyLight::new(115), KeyCode::new(251, 115), KeyName::Num4),
        KeyInfo::new(KeyLight::new(116), KeyCode::new(251, 116), KeyName::Num1),
        KeyInfo::new(KeyLight::new(117), KeyCode::new(251, 133), KeyName::Num0),
        KeyInfo::new(
            KeyLight::new(119),
            KeyCode::new(251, 121),
            KeyName::NumDivide,
        ),
        KeyInfo::new(KeyLight::new(120), KeyCode::new(251, 122), KeyName::Num8),
        KeyInfo::new(KeyLight::new(121), KeyCode::new(251, 123), KeyName::Num5),
        KeyInfo::new(KeyLight::new(122), KeyCode::new(251, 124), KeyName::Num2),
        KeyInfo::new(
            KeyLight::new(124),
            KeyCode::new(251, 129),
            KeyName::NumMultiply,
        ),
        KeyInfo::new(KeyLight::new(125), KeyCode::new(251, 130), KeyName::Num9),
        KeyInfo::new(KeyLight::new(126), KeyCode::new(251, 131), KeyName::Num6),
        KeyInfo::new(KeyLight::new(127), KeyCode::new(251, 132), KeyName::Num3),
        KeyInfo::new(KeyLight::new(128), KeyCode::new(251, 141), KeyName::NumDot),
        KeyInfo::new(
            KeyLight::new(129),
            KeyCode::new(251, 137),
            KeyName::NumMinus,
        ),
        KeyInfo::new(KeyLight::new(130), KeyCode::new(251, 138), KeyName::NumPlus),
        KeyInfo::new(
            KeyLight::new(131),
            KeyCode::new(251, 140),
            KeyName::NumEnter,
        ),
    ]
}

/// Swiss French Layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct LayoutFrCh;

impl LayoutFrCh {
    /// Initialize a configuration
    pub const fn new() -> Self {
        Self
    }

    /// Contains the layout.
    const LAYOUT: [KeyInfo; 107] = get_layout_info_fr_ch();

    /// Get the array of [`KeyInfo`]
    pub const fn layout_key_info() -> &'static [KeyInfo; 107] {
        &Self::LAYOUT
    }
}

impl Default for LayoutFrCh {
    fn default() -> Self {
        Self::new()
    }
}

impl Layout for LayoutFrCh {
    fn layout(&self) -> &[KeyInfo] {
        Self::layout_key_info()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// layout search function
    #[test]
    fn search_key_layout_ch_fr() {
        let layout = LayoutFrCh::new();
        for key_info in layout.layout().iter() {
            assert_eq!(
                layout.find_from_key_light(*key_info.key_code_light()),
                Some(key_info)
            );
            assert_eq!(
                layout.find_from_key_code(*key_info.key_code_press()),
                Some(key_info)
            );
            assert_eq!(
                layout.find_from_key_name(*key_info.key_name()),
                Some(key_info)
            );
        }
    }
}
