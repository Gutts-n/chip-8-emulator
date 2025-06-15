use std::fmt;

pub const MEMORY_SIZE: usize = 4096;
const FONT_START_ADDRESS: usize = 0x50;

pub struct Memory {
    memory: [u8; MEMORY_SIZE],
}

pub trait MemoryTrait {
    fn write(&mut self, position: usize, value: u8) -> bool;
    fn retrieve(&mut self, position: usize) -> u8;
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Memory ({} bytes)", MEMORY_SIZE)?;
        writeln!(f, "====================")?;

        // Display interpreter area (0x000 - 0x1FF)
        writeln!(f, "Interpreter Area (0x000-0x1FF / 0-511): Protected")?;

        // Display font data area
        writeln!(
            f,
            "Font Data (0x{:03X}-0x{:03X} / {}-{}):",
            FONT_START_ADDRESS,
            FONT_START_ADDRESS + 79,
            FONT_START_ADDRESS,
            FONT_START_ADDRESS + 79
        )?;

        // Show first few bytes of font data as example
        write!(f, "  First 16 bytes: ")?;
        for i in 0..16 {
            if i < 80 && FONT_START_ADDRESS + i < MEMORY_SIZE {
                write!(f, "{:02X} ", self.memory[FONT_START_ADDRESS + i])?;
            }
        }
        writeln!(f)?;

        // Display program area (0x200 onwards)
        writeln!(
            f,
            "Program Area (0x200-0x{:03X} / 512-{}):",
            MEMORY_SIZE - 1,
            MEMORY_SIZE - 1
        )?;

        // Find the last non-zero byte in program area to avoid showing empty memory
        let mut last_used = 0x200;
        for i in (0x200..MEMORY_SIZE).rev() {
            if self.memory[i] != 0 {
                last_used = i;
                break;
            }
        }

        if last_used > 0x200 {
            writeln!(
                f,
                "  Used memory up to: 0x{:03X} ({})",
                last_used, last_used
            )?;
            writeln!(
                f,
                "  Sample data (0x200-0x{:03X} / 512-{}):",
                std::cmp::min(0x20F, last_used),
                std::cmp::min(0x20F, last_used)
            )?;

            // Show program data in hex dump format
            for row in (0x200..=std::cmp::min(0x20F, last_used)).step_by(16) {
                write!(f, "  {:03X} ({:4}): ", row, row)?;
                for col in 0..16 {
                    if row + col <= last_used && row + col < MEMORY_SIZE {
                        write!(f, "{:02X} ", self.memory[row + col])?;
                    } else {
                        write!(f, "   ")?;
                    }
                }
                writeln!(f)?;
            }
        } else {
            writeln!(f, "  No program data loaded")?;
        }

        Ok(())
    }
}
impl MemoryTrait for Memory {
    fn write(&mut self, position: usize, value: u8) -> bool {
        if position >= MEMORY_SIZE {
            println!("Position not supported on the memory");
            return false;
        }
        // The CHIP used the 0 to 512 indexes to allocate the interpreter
        if position <= 0x1FF {
            println!("Memory allocated to the interpreter");
            return false;
        }
        self.memory[position] = value;
        return true;
    }

    fn retrieve(&mut self, position: usize) -> u8 {
        if position >= MEMORY_SIZE {
            println!("Position not supported on the memory");
            return 0x00;
        }
        self.memory[position]
    }
}

impl Memory {
    pub fn new() -> Memory {
        let mut memory = Memory {
            memory: [0; MEMORY_SIZE],
        };
        memory.load_font();
        memory
    }

    fn load_font(&mut self) {
        const FONT_DATA: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // Load font data into memory starting at address 0x50
        for (i, &byte) in FONT_DATA.iter().enumerate() {
            self.memory[FONT_START_ADDRESS + i] = byte;
        }
    }
}

