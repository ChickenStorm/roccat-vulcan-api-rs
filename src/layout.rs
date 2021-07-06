//! Defines the layout of the keyboard and differents key infos

use std::{
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    convert::From,
    fmt,
    string::String,
};

pub mod layout_fr_ch;

/// Key press event.
#[derive(Clone, Debug)]
pub struct Keypress {
    key_code: KeyCodePress,
    is_pressed: bool,
}

impl Keypress {
    /// read key press from buffer hid read buffer
    pub fn new_from_buffer(buffer: &[u8; 5]) -> Self {
        Self {
            key_code: KeyCodePress::new(buffer[2], buffer[3]),
            is_pressed: {
                match buffer[2] {
                    10 => buffer[4] == 0, // for some reason the caps loc signal is inverted
                    _ => buffer[4] != 0,
                }
            },
        }
    }

    pub fn new(key_code: KeyCodePress, is_pressed: bool) -> Self {
        Self {
            key_code,
            is_pressed,
        }
    }

    pub fn key_code(&self) -> &KeyCodePress {
        &self.key_code
    }

    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

impl From<Keypress> for KeyCodePress {
    fn from(d: Keypress) -> Self {
        d.key_code
    }
}

/// structur of data to incode the key when a key press is read form hid device.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct KeyCodePress {
    first_u8: u8,
    seconde_u8: u8,
}

impl KeyCodePress {
    pub fn new(first_u8: u8, seconde_u8: u8) -> Self {
        Self {
            first_u8,
            seconde_u8,
        }
    }

    pub fn first_u8(&self) -> u8 {
        self.first_u8
    }

    pub fn seconde_u8(&self) -> u8 {
        self.seconde_u8
    }
}

impl From<[u8; 2]> for KeyCodePress {
    fn from(array: [u8; 2]) -> Self {
        KeyCodePress::new(array[0], array[1])
    }
}

impl From<(u8, u8)> for KeyCodePress {
    fn from(tuple: (u8, u8)) -> Self {
        let (a, b) = tuple;
        KeyCodePress::new(a, b)
    }
}

impl From<KeyCodePress> for [u8; 2] {
    fn from(d: KeyCodePress) -> Self {
        [d.first_u8(), d.seconde_u8()]
    }
}

/// Liste of keys. Some key might be missing.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Key {
    /// key not listed or other key
    Unbound,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Break,
    WheelUp,
    WheelDown,
    /// §
    Section,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    /// '
    Apostrophe,
    /// ^
    Caret,
    BackSpace,
    Tab,
    Q,
    W,
    E,
    R,
    T,
    Z,
    U,
    I,
    O,
    P,
    EGrave,
    /// ¨
    Diaeresis,
    CapsLock,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    EAcute,
    AGrave,
    Dolar,
    Enter,
    LeftShift,
    LessThan,
    Y,
    X,
    C,
    V,
    B,
    N,
    M,
    Comma,
    Dot,
    Dash,
    RightShift,
    LeftControl,
    /// Meta left
    Super,
    Alt,
    Space,
    AltGr,
    Function,
    /// Meta right
    Menu,
    RightControl,
    Insert,
    Home,
    PageUp,
    PageDown,
    Delete,
    End,
    NumLock,
    NumDivide,
    NumMultiply,
    NumMinus,
    NumPlus,
    NumEnter,
    NumDot,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

type KeyCodeLight = u8; // might change to usize

///  associative data for a key.
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct KeyInfo {
    key_code_light: KeyCodeLight,
    key_code_press: KeyCodePress,
    key: Key,
    key_string: String,
}

impl KeyInfo {
    pub fn new(
        key_code_light: KeyCodeLight,
        key_code_press: KeyCodePress,
        key_string: String,
        key: Key,
    ) -> Self {
        Self {
            key_code_light,
            key_code_press,
            key,
            key_string,
        }
    }

    /// create a key info using the `key.to_string()` for [`Key`]
    pub fn new_from_key(
        key_code_light: KeyCodeLight,
        key_code_press: KeyCodePress,
        key: Key,
    ) -> Self {
        Self {
            key_code_light,
            key_code_press,
            key_string: key.to_string(),
            key,
        }
    }

    pub fn key_code_light(&self) -> &KeyCodeLight {
        &self.key_code_light
    }

    pub fn key_code_press(&self) -> &KeyCodePress {
        &self.key_code_press
    }

    pub fn key_string(&self) -> &String {
        &self.key_string
    }

    pub fn key(&self) -> &Key {
        &self.key
    }
}

impl From<(KeyCodeLight, KeyCodePress, String, Key)> for KeyInfo {
    fn from(tuple: (KeyCodeLight, KeyCodePress, String, Key)) -> Self {
        let (key_code_light, key_code_press, key_string, key) = tuple;
        KeyInfo::new(key_code_light, key_code_press, key_string, key)
    }
}

/// Information to encode a layout, get key info from differents way to encode a key.
pub trait Layout {
    fn find_key_info_from_light(&self, key_code_light: &KeyCodeLight) -> Option<&KeyInfo>;
    fn find_key_info_from_press_code(&self, key_code_press: &KeyCodePress) -> Option<&KeyInfo>;
    fn find_key_info_from_key(&self, key: &Key) -> Option<&KeyInfo>;
    fn find_key_info_from_string(&self, string: &str) -> Option<&KeyInfo>;
}
