pub struct Display {
    pixels: [[bool; 64]; 32],
}

pub trait DisplayTrait {
    fn refresh(&mut self) -> bool;
    fn draw(&mut self, position: usize) -> bool;
    fn clear(&mut self) -> bool;
}

impl DisplayTrait for Display {
    fn refresh(&mut self) -> bool {
        false
    }

    fn draw(&mut self, position: usize) -> bool {
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
}

impl Display {
    pub fn new() -> Display {
        let pixels = [[false; 64]; 32];
        let display = Display { pixels: pixels };
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
