use crate::memory::{MemoryTrait, SharedMemory};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

pub struct Display {
    pixels: [[bool; 64]; 32],
    memory: SharedMemory,
}

pub trait DisplayTrait {
    fn refresh(&mut self) -> bool;
    fn draw(&mut self, x: usize, y: usize, num_bytes: usize, i_register: usize);
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

    fn draw(&mut self, x: usize, y: usize, num_bytes: usize, i_register: usize) {
        // The num_bytes is the total of values that will take from the memory from the index presented
        // inside of the i_register

        // Loop through each row of the sprite (each byte represents one 8-pixel row)
        for row in 0..num_bytes {
            // Get one byte of sprite data from memory (8 pixels worth)
            let sprite_byte = self.memory.borrow().retrieve(i_register + row);

            // Calculate which screen row to draw on, wrapping if it goes past bottom (32 rows total)
            let screen_row = (y + row) % 32;

            // Process each of the 8 bits in this sprite byte (each bit = one pixel)
            for bit_position in 0..8 {
                // Extract the specific bit from the sprite byte (leftmost bit first)
                let sprite_pixel = (sprite_byte >> (7 - bit_position)) & 1;

                // Calculate which screen column to draw on, wrapping if it goes past right edge (64 columns total)
                let screen_column = (x + bit_position) % 64;

                // XOR the sprite pixel with the screen pixel (CHIP-8's drawing method)
                // If sprite bit is 1, flip the screen pixel; if sprite bit is 0, leave unchanged
                self.pixels[screen_row][screen_column] ^= sprite_pixel == 1;
            }
        }

        // Display the updated screen
        self.print_with_crossterm();
    }
}

impl Display {
    pub fn new(memory: SharedMemory) -> Display {
        let pixels = [[false; 64]; 32];
        let display = Display { pixels, memory };
        display
    }

    // print without lib
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

    fn print_with_crossterm(&self) {
        let mut stdout = stdout();

        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(Hide).unwrap();

        for (row_idx, row) in self.pixels.iter().enumerate() {
            stdout.execute(MoveTo(0, row_idx as u16)).unwrap();

            let row_string: String = row
                .iter()
                .map(|&pixel| if pixel { '█' } else { '·' })
                .collect();

            print!("{}", row_string);
        }

        stdout.execute(Show).unwrap();
        stdout.flush().unwrap();
    }
}
