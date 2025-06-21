pub struct Stack {
    addresses: [u16; 50],
    size: usize,
}

pub trait StackTrait {
    fn push(&mut self, address: u16) -> bool;
    fn pop(&mut self) -> u16;
    fn peek(&mut self) -> u16;
}

impl StackTrait for Stack {
    fn push(&mut self, address: u16) -> bool {
        self.addresses[self.size] = address;
        self.size = self.size + 1;
        true
    }

    fn pop(&mut self) -> u16 {
        if self.size == 0 {
            return 0x00;
        }

        let address = self.addresses[self.size - 1];
        self.size = self.size - 1;
        address
    }

    fn peek(&mut self) -> u16 {
        self.addresses[self.size]
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            size: 0,
            addresses: [0; 50],
        }
    }
}
