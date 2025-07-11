use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};
use std::process::exit;
use std::time::Duration;

#[derive(Eq, Hash, PartialEq)]
pub enum CosmacVIPKey {
    Key1,
    Key2,
    Key3,
    KeyC,
    Key4,
    Key5,
    Key6,
    KeyD,
    Key7,
    Key8,
    Key9,
    KeyE,
    KeyA,
    Key0,
    KeyB,
    KeyF,
}

impl CosmacVIPKey {
    pub fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            b'1' => Some(Self::Key1),
            b'2' => Some(Self::Key2),
            b'3' => Some(Self::Key3),
            b'c' | b'C' => Some(Self::KeyC),
            b'4' => Some(Self::Key4),
            b'5' => Some(Self::Key5),
            b'6' => Some(Self::Key6),
            b'd' | b'D' => Some(Self::KeyD),
            b'7' => Some(Self::Key7),
            b'8' => Some(Self::Key8),
            b'9' => Some(Self::Key9),
            b'e' | b'E' => Some(Self::KeyE),
            b'a' | b'A' => Some(Self::KeyA),
            b'0' => Some(Self::Key0),
            b'b' | b'B' => Some(Self::KeyB),
            b'f' | b'F' => Some(Self::KeyF),
            _ => None,
        }
    }
}

pub struct Keyboard {
    keys: HashMap<CosmacVIPKey, bool>,
}

impl Display for CosmacVIPKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let key_str = match self {
            CosmacVIPKey::Key1 => "1",
            CosmacVIPKey::Key2 => "2",
            CosmacVIPKey::Key3 => "3",
            CosmacVIPKey::KeyC => "C",
            CosmacVIPKey::Key4 => "4",
            CosmacVIPKey::Key5 => "5",
            CosmacVIPKey::Key6 => "6",
            CosmacVIPKey::KeyD => "D",
            CosmacVIPKey::Key7 => "7",
            CosmacVIPKey::Key8 => "8",
            CosmacVIPKey::Key9 => "9",
            CosmacVIPKey::KeyE => "E",
            CosmacVIPKey::KeyA => "A",
            CosmacVIPKey::Key0 => "0",
            CosmacVIPKey::KeyB => "B",
            CosmacVIPKey::KeyF => "F",
        };
        write!(f, "{key_str}")
    }
}

pub trait KeyboardTrait {
    fn process_any_input(&mut self);
    fn is_key_pressed(&mut self, byte: u8) -> bool;
    fn map_key_to_chip8(&self, key: KeyCode) -> Option<CosmacVIPKey>;
    fn get_key_pressed(&self) -> u8;
}

impl KeyboardTrait for Keyboard {
    fn process_any_input(&mut self) {
        let mut ctrl_c_pressed = false;

        while poll(Duration::from_millis(0)).unwrap() {
            if let Ok(Event::Key(KeyEvent {
                kind,
                state,
                code,
                modifiers,
            })) = read()
            {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    match code {
                        KeyCode::Char('c') => {
                            ctrl_c_pressed = true;
                        }
                        _ => {}
                    }

                    if ctrl_c_pressed {
                        println!("Ctrl+C pressed. Exiting...");
                        exit(0);
                    }
                }

                if let Some(chip8_key) = self.map_key_to_chip8(code) {
                    for (key, value) in self.keys.iter_mut() {
                        *value = key == &chip8_key
                    }
                }
            }
        }
    }

    fn is_key_pressed(&mut self, byte: u8) -> bool {
        if let Some(key) = CosmacVIPKey::from_u8(byte) {
            return self.keys.get(&key).is_none();
        } else {
            return false;
        }
    }

    fn map_key_to_chip8(&self, key: KeyCode) -> Option<CosmacVIPKey> {
        match key {
            KeyCode::Char(c) => match c.to_ascii_lowercase() {
                '1' => Some(CosmacVIPKey::Key1),
                '2' => Some(CosmacVIPKey::Key2),
                '3' => Some(CosmacVIPKey::Key3),
                '4' => Some(CosmacVIPKey::KeyC),
                'q' => Some(CosmacVIPKey::Key4),
                'w' => Some(CosmacVIPKey::Key5),
                'e' => Some(CosmacVIPKey::Key6),
                'r' => Some(CosmacVIPKey::KeyD),
                'a' => Some(CosmacVIPKey::Key7),
                's' => Some(CosmacVIPKey::Key8),
                'd' => Some(CosmacVIPKey::Key9),
                'f' => Some(CosmacVIPKey::KeyE),
                'z' => Some(CosmacVIPKey::KeyA),
                'x' => Some(CosmacVIPKey::Key0),
                'c' => Some(CosmacVIPKey::KeyB),
                'v' => Some(CosmacVIPKey::KeyF),
                _ => None,
            },
            _ => None,
        }
    }

    fn get_key_pressed(&self) -> Option<CosmacVIPKey> {
        for (key, value) in self.keys.iter_mut() {
            match value {
                true => return Some(key),
                false => return None,
            }
        }
    }
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let keys = HashMap::new();
        let keyboard = Keyboard { keys };
        keyboard
    }
}
