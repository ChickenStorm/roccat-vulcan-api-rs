//! Defines the layout for Swiss French layout

use std::fmt::{Display, Formatter};

#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

use super::{KeyCode, KeyInfo, KeyLight, KeyName, Layout, Position};

/// get Layout key associative array
const fn layout_info_fr_ch() -> [KeyInfo; 107] {
    [
        KeyInfo::new(
            KeyLight::new(3),
            KeyCode::new(10, 57),
            KeyName::CapsLock,
            Position::new(0_f64, 2_f64),
        ),
        // note that the index for the key light i out of bound for the wheel up and down
        KeyInfo::new(
            KeyLight::new(250),
            KeyCode::new(204, 1),
            KeyName::WheelUp,
            Position::new(22_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(251),
            KeyCode::new(204, 255),
            KeyName::WheelDown,
            Position::new(22_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(0),
            KeyCode::new(251, 17),
            KeyName::Escape,
            Position::new(0_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(1),
            KeyCode::new(251, 18),
            KeyName::Section,
            Position::new(0_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(2),
            KeyCode::new(251, 20),
            KeyName::Tab,
            Position::new(0_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(4),
            KeyCode::new(251, 22),
            KeyName::LeftShift,
            Position::new(0_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(5),
            KeyCode::new(251, 23),
            KeyName::LeftControl,
            Position::new(0_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(6),
            KeyCode::new(251, 19),
            KeyName::Key1,
            Position::new(1_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(7),
            KeyCode::new(251, 26),
            KeyName::Q,
            Position::new(1_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(8),
            KeyCode::new(251, 28),
            KeyName::A,
            Position::new(1_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(9),
            KeyCode::new(251, 29),
            KeyName::LessThan,
            Position::new(1_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(10),
            KeyCode::new(251, 31),
            KeyName::Super,
            Position::new(1_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(11),
            KeyCode::new(251, 16),
            KeyName::F1,
            Position::new(2_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(12),
            KeyCode::new(251, 25),
            KeyName::Key2,
            Position::new(2_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(13),
            KeyCode::new(251, 27),
            KeyName::W,
            Position::new(2_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(14),
            KeyCode::new(251, 37),
            KeyName::S,
            Position::new(2_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(15),
            KeyCode::new(251, 38),
            KeyName::Y,
            Position::new(2_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(16),
            KeyCode::new(251, 39),
            KeyName::Alt,
            Position::new(2_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(17),
            KeyCode::new(251, 24),
            KeyName::F2,
            Position::new(3_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(18),
            KeyCode::new(251, 34),
            KeyName::Key3,
            Position::new(3_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(19),
            KeyCode::new(251, 36),
            KeyName::E,
            Position::new(3_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(20),
            KeyCode::new(251, 44),
            KeyName::D,
            Position::new(3_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(21),
            KeyCode::new(251, 45),
            KeyName::X,
            Position::new(3_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(23),
            KeyCode::new(251, 33),
            KeyName::F3,
            Position::new(4_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(24),
            KeyCode::new(251, 35),
            KeyName::Key4,
            Position::new(4_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(25),
            KeyCode::new(251, 43),
            KeyName::R,
            Position::new(4_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(26),
            KeyCode::new(251, 53),
            KeyName::F,
            Position::new(4_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(27),
            KeyCode::new(251, 46),
            KeyName::C,
            Position::new(4_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(28),
            KeyCode::new(251, 32),
            KeyName::F4,
            Position::new(5_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(29),
            KeyCode::new(251, 42),
            KeyName::Key5,
            Position::new(5_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(30),
            KeyCode::new(251, 51),
            KeyName::T,
            Position::new(5_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(31),
            KeyCode::new(251, 52),
            KeyName::G,
            Position::new(5_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(32),
            KeyCode::new(251, 54),
            KeyName::V,
            Position::new(5_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(33),
            KeyCode::new(251, 41),
            KeyName::Key6,
            Position::new(6_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(34),
            KeyCode::new(251, 59),
            KeyName::Z,
            Position::new(6_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(35),
            KeyCode::new(251, 61),
            KeyName::H,
            Position::new(6_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(36),
            KeyCode::new(251, 62),
            KeyName::B,
            Position::new(6_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(37),
            KeyCode::new(251, 63),
            KeyName::Space,
            Position::new(6_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(48),
            KeyCode::new(251, 40),
            KeyName::F5,
            Position::new(7_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(49),
            KeyCode::new(251, 49),
            KeyName::Key7,
            Position::new(7_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(50),
            KeyCode::new(251, 60),
            KeyName::U,
            Position::new(7_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(51),
            KeyCode::new(251, 68),
            KeyName::J,
            Position::new(7_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(52),
            KeyCode::new(251, 71),
            KeyName::N,
            Position::new(7_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(53),
            KeyCode::new(251, 48),
            KeyName::F6,
            Position::new(8_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(54),
            KeyCode::new(251, 66),
            KeyName::Key8,
            Position::new(8_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(55),
            KeyCode::new(251, 67),
            KeyName::I,
            Position::new(8_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(56),
            KeyCode::new(251, 69),
            KeyName::K,
            Position::new(8_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(57),
            KeyCode::new(251, 70),
            KeyName::M,
            Position::new(8_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(59),
            KeyCode::new(251, 56),
            KeyName::F7,
            Position::new(9_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(60),
            KeyCode::new(251, 65),
            KeyName::Key9,
            Position::new(9_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(61),
            KeyCode::new(251, 76),
            KeyName::O,
            Position::new(9_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(62),
            KeyCode::new(251, 77),
            KeyName::L,
            Position::new(9_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(63),
            KeyCode::new(251, 78),
            KeyName::Comma,
            Position::new(9_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(65),
            KeyCode::new(251, 57),
            KeyName::F8,
            Position::new(10_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(66),
            KeyCode::new(251, 74),
            KeyName::Key0,
            Position::new(10_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(67),
            KeyCode::new(251, 84),
            KeyName::P,
            Position::new(10_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(68),
            KeyCode::new(251, 85),
            KeyName::EAcute,
            Position::new(10_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(69),
            KeyCode::new(251, 86),
            KeyName::Dot,
            Position::new(10_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(70),
            KeyCode::new(251, 103),
            KeyName::AltGr,
            Position::new(10_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(72),
            KeyCode::new(251, 75),
            KeyName::Apostrophe,
            Position::new(11_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(73),
            KeyCode::new(251, 91),
            KeyName::EGrave,
            Position::new(11_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(74),
            KeyCode::new(251, 93),
            KeyName::AGrave,
            Position::new(11_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(75),
            KeyCode::new(251, 94),
            KeyName::Dash,
            Position::new(11_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(76),
            KeyCode::new(251, 119),
            KeyName::Function,
            Position::new(11_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(78),
            KeyCode::new(251, 64),
            KeyName::F9,
            Position::new(12_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(79),
            KeyCode::new(251, 83),
            KeyName::Caret,
            Position::new(13_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(80),
            KeyCode::new(251, 92),
            KeyName::Diaeresis,
            Position::new(13_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(82),
            KeyCode::new(251, 110),
            KeyName::RightShift,
            Position::new(14_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(83),
            KeyCode::new(251, 127),
            KeyName::Menu,
            Position::new(13_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(84),
            KeyCode::new(251, 72),
            KeyName::F10,
            Position::new(13_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(85),
            KeyCode::new(251, 80),
            KeyName::F11,
            Position::new(14_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(86),
            KeyCode::new(251, 81),
            KeyName::F12,
            Position::new(15_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(87),
            KeyCode::new(251, 73),
            KeyName::BackSpace,
            Position::new(15_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(88),
            KeyCode::new(251, 107),
            KeyName::Enter,
            Position::new(15_f64, 2.5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(89),
            KeyCode::new(251, 135),
            KeyName::RightControl,
            Position::new(15_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(96),
            KeyCode::new(251, 100),
            KeyName::Dolar,
            Position::new(13_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(99),
            KeyCode::new(251, 88),
            KeyName::PrintScreen,
            Position::new(16_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(100),
            KeyCode::new(251, 89),
            KeyName::Insert,
            Position::new(16_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(101),
            KeyCode::new(251, 90),
            KeyName::Delete,
            Position::new(16_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(102),
            KeyCode::new(251, 109),
            KeyName::ArrowLeft,
            Position::new(16_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(103),
            KeyCode::new(251, 96),
            KeyName::ScrollLock,
            Position::new(17_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(104),
            KeyCode::new(251, 97),
            KeyName::Home,
            Position::new(17_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(105),
            KeyCode::new(251, 98),
            KeyName::End,
            Position::new(17_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(106),
            KeyCode::new(251, 108),
            KeyName::ArrowUp,
            Position::new(17_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(107),
            KeyCode::new(251, 117),
            KeyName::ArrowDown,
            Position::new(17_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(108),
            KeyCode::new(251, 104),
            KeyName::Break,
            Position::new(18_f64, 5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(109),
            KeyCode::new(251, 105),
            KeyName::PageUp,
            Position::new(18_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(110),
            KeyCode::new(251, 106),
            KeyName::PageDown,
            Position::new(18_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(111),
            KeyCode::new(251, 125),
            KeyName::ArrowRight,
            Position::new(18_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(113),
            KeyCode::new(251, 113),
            KeyName::NumLock,
            Position::new(19_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(114),
            KeyCode::new(251, 114),
            KeyName::Num7,
            Position::new(19_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(115),
            KeyCode::new(251, 115),
            KeyName::Num4,
            Position::new(19_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(116),
            KeyCode::new(251, 116),
            KeyName::Num1,
            Position::new(19_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(117),
            KeyCode::new(251, 133),
            KeyName::Num0,
            Position::new(19.5_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(119),
            KeyCode::new(251, 121),
            KeyName::NumDivide,
            Position::new(20_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(120),
            KeyCode::new(251, 122),
            KeyName::Num8,
            Position::new(20_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(121),
            KeyCode::new(251, 123),
            KeyName::Num5,
            Position::new(20_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(122),
            KeyCode::new(251, 124),
            KeyName::Num2,
            Position::new(20_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(124),
            KeyCode::new(251, 129),
            KeyName::NumMultiply,
            Position::new(21_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(125),
            KeyCode::new(251, 130),
            KeyName::Num9,
            Position::new(21_f64, 3_f64),
        ),
        KeyInfo::new(
            KeyLight::new(126),
            KeyCode::new(251, 131),
            KeyName::Num6,
            Position::new(21_f64, 2_f64),
        ),
        KeyInfo::new(
            KeyLight::new(127),
            KeyCode::new(251, 132),
            KeyName::Num3,
            Position::new(21_f64, 1_f64),
        ),
        KeyInfo::new(
            KeyLight::new(128),
            KeyCode::new(251, 141),
            KeyName::NumDot,
            Position::new(21_f64, 0_f64),
        ),
        KeyInfo::new(
            KeyLight::new(129),
            KeyCode::new(251, 137),
            KeyName::NumMinus,
            Position::new(22_f64, 4_f64),
        ),
        KeyInfo::new(
            KeyLight::new(130),
            KeyCode::new(251, 138),
            KeyName::NumPlus,
            Position::new(22_f64, 2.5_f64),
        ),
        KeyInfo::new(
            KeyLight::new(131),
            KeyCode::new(251, 140),
            KeyName::NumEnter,
            Position::new(22_f64, 0.5_f64),
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
    const LAYOUT: [KeyInfo; 107] = layout_info_fr_ch();

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

impl Display for LayoutFrCh {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // TODO
        write!(f, "swiss french layout")
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

    #[test]
    fn layout_get_gen() {
        let layout = layout_info_fr_ch();
        assert_eq!(layout, LayoutFrCh::LAYOUT);
        assert_eq!(&layout, LayoutFrCh::layout_key_info());
        assert_eq!(&layout, LayoutFrCh::default().layout());
    }

    #[test]
    fn layout_gen_key() {
        let layout = LayoutFrCh::default();
        for i in 1_u8..=21_u8 {
            assert_has_key_light(&layout, i);
        }
        for i in 23_u8..=37_u8 {
            assert_has_key_light(&layout, i);
        }
        for i in 48_u8..=57 {
            assert_has_key_light(&layout, i);
        }
        for i in 59..=63 {
            assert_has_key_light(&layout, i);
        }
        for i in 65..=70 {
            assert_has_key_light(&layout, i);
        }
        for i in 72..=76 {
            assert_has_key_light(&layout, i);
        }
        for i in 78..=80 {
            assert_has_key_light(&layout, i);
        }
        for i in 82..=89 {
            assert_has_key_light(&layout, i);
        }
        assert_has_key_light(&layout, 96);
        for i in 99..=111 {
            assert_has_key_light(&layout, i);
        }
        for i in 113..=117 {
            assert_has_key_light(&layout, i);
        }
        for i in 119..=122 {
            assert_has_key_light(&layout, i);
        }
        for i in 124..=131 {
            assert_has_key_light(&layout, i);
        }
        for i in 250..=251 {
            assert_has_key_light(&layout, i);
        }
    }

    fn assert_has_key_light(layout: &impl Layout, code: u8) {
        let key_light = KeyLight::new(code);
        assert!(
            layout.find_from_key_light(key_light).is_some(),
            "missing light code {}",
            key_light
        );
    }
}
