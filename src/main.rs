mod display;
mod memory;
mod stack;

use memory::MEMORY_SIZE;
use memory::{Memory, MemoryTrait};
use std::fs;
use std::io;

const PROGRAM_START: usize = 0x200;
const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START;

pub fn load_rom(file_path: &str, memory: &mut Memory) -> Result<usize, String> {
    // Read the ROM file
    let rom_data = fs::read(file_path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => format!("ROM file not found: {}", file_path),
        io::ErrorKind::PermissionDenied => format!("Permission denied reading: {}", file_path),
        _ => format!("Failed to read ROM file: {}", e),
    })?;

    if rom_data.len() > MAX_ROM_SIZE {
        return Err(format!(
            "ROM too large: {} bytes (max: {} bytes)",
            rom_data.len(),
            MAX_ROM_SIZE
        ));
    }

    if rom_data.is_empty() {
        return Err("ROM file is empty".to_string());
    }

    for (i, &byte) in rom_data.iter().enumerate() {
        memory.write(PROGRAM_START + i, byte);
    }

    println!("Loaded ROM: {} ({} bytes)", file_path, rom_data.len());
    println!("Memory: {}", memory);
    Ok(rom_data.len())
}

fn main() {
    let result = load_rom("IBM Logo.ch8", &mut Memory::new());
    match result {
        Ok(_) => println!("Successfully loaded the ROM"),
        Err(e) => panic!("Failed to load the ROM: {}", e),
    }
}
