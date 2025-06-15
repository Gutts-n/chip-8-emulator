struct Display {
    pixels: [[bool; 64]; 32],
}

trait DisplayTrait {
    fn refresh(&mut self) -> bool;
    fn draw(&mut self, position: usize) -> bool;
}

impl DisplayTrait for Display {
    fn refresh(&mut self) -> bool {
        false
    }

    fn draw(&mut self, position: usize) -> bool {
        false
    }
}
