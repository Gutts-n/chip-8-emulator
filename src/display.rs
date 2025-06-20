use crate::memory::SharedMemory;

pub struct Display {
    pixels: [[bool; 64]; 32],
    memory: SharedMemory,
}

pub trait DisplayTrait {
    fn refresh(&mut self) -> bool;
    fn draw(&mut self, x: usize, y: usize, number_of_pixels_to_turn_on_or_off: u8) -> bool;
    fn clear(&mut self) -> bool;
}

impl DisplayTrait for Display {
    fn refresh(&mut self) -> bool {
        false
    }

    fn clear(&mut self) -> bool {
        for (_, row) in self.pixels.iter_mut().enumerate() {
            for j in 0..row.len() {
                row[j] = false;
            }
        }

        return true;
    }

    fn draw(&mut self, x: usize, y: usize, number_of_pixels_to_turn_on_or_off: u8) -> bool {
        true
    }
}

impl Display {
    pub fn new(memory: SharedMemory) -> Display {
        let pixels = [[false; 64]; 32];
        let display = Display {
            pixels: pixels,
            memory,
        };
        display
    }

    pub fn print(&self) {
        println!("  ");
        println!("////");
        println!("  ");
        for row in self.pixels.iter() {
            for &pixel in row.iter() {
                let symbol = if pixel { '█' } else { '_' };
                print!("{}", symbol);
            }
            println!();
        }
    }
}
