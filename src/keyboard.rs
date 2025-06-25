use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
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

    fn is_key_pressed(&mut self, byte: u8) -> bool {
        true
    }
}

impl Keyboard {
    pub fn new() -> Keyboard {
        let keys = HashMap::new();
        let keyboard = Keyboard { keys };
        keyboard
    }
}
