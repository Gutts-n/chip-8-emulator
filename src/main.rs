mod display;
mod memory;
mod stack;
use display::{Display, DisplayTrait};
use memory::MEMORY_SIZE;
use memory::{Memory, MemoryTrait};
use std::fs;
use std::io;

const PROGRAM_START: usize = 0x200;
const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START;
const CLEAR: u16 = 0x00E0; // Clear screen (exact match, no mask needed because it doesn't send
// args)
const JUMP_TO_NNN: u16 = 0x1000; // 1NNN: Jump to address NNN 
const SET_NN_TO_VX: u16 = 0x6000; // 6XNN: Set VX to NN 
const SUM_NN_TO_VX: u16 = 0x7000; // 7XNN: Add NN to VX 
const SET_NNN_TO_I: u16 = 0xA000; // ANNN: Set I to NNN 
const DRAW: u16 = 0xD000; // DXYN: Draw sprite at (VX, VY) with height N 

pub fn load_rom(file_path: &str, memory: &mut Memory) -> Result<Vec<u8>, String> {
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
    Ok(rom_data)
}

fn process_instructions(memory: &mut Memory, display: &mut Display) {
    let mut program_counter = PROGRAM_START;
    let mut registers = [0x00; 16];
    let mut i_register = 0x00;
    while true {
        // display.print();
        let first_byte = memory.retrieve(program_counter) as u16;
        let second_byte = memory.retrieve(program_counter + 1) as u16;
        let instruction = (first_byte << 8) + second_byte;
        program_counter = program_counter + 2;
        if program_counter >= MEMORY_SIZE {
            break;
        }
        // THIS IS GETTING ONLY THE TYPE OF THE INSTRUCTION AS EACH CALL FOR THE CPU WILL HAVE ARGS
        // WITH IT SO IT MUST ISOLATE ONLY THE TYPE OF THE CALL AND PROCESS THE RANDOM ARGS INSIDE
        // OF THE BLOCK THIS MASK IS ISOLATING ONLY THE FIRST 4 BITS OF THE INSTRUCTION
        // MASK: 1111 0000 0000 0000
        match instruction & 0xF000 {
            CLEAR => {
                println!("Clearing the display");
                display.clear();
            }
            SUM_NN_TO_VX => {
                // Instruction: 0110 0010 0011 0111 (0x6237)
                // Mask 0x0F00: 0000 1111 0000 0000
                // ─────────────────────
                // Result:      0000 0010 0000 0000 = 0x0200 that is not the value expected, we
                // have to shift it 8 bits to the right to ensure that we are getting the real
                // argument:
                // 0x0200 = 0000 0010 0000 0000
                // >> 8   = 0000 0000 0000 0010 = 0x0002
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                // Instruction: 0110 0010 0011 0111
                // Mask 0x00FF: 0000 0000 1111 1111
                // ─────────────────────
                // Result: 0000 0000 0011 0111 = 0x0037
                let value = instruction & 0x00FF;
                println!(
                    "Summing the value {} to the register {} that has the value {}",
                    value, register_index, registers[register_index]
                );
                registers[register_index as usize] = registers[register_index] + value;
            }
            SET_NNN_TO_I => {
                // Instruction: 0110 0010 0011 0111
                // Mask 0x0FFF: 0000 1111 1111 1111
                // _______________________________________________________________
                // Result: 0000 0000 0011 0111 = 0x0037
                let value = (instruction & 0x0FFF) as usize;
                println!("Setting the value {} to the register I", value);
                i_register = value;
            }
            SET_NN_TO_VX => {
                // Instruction: 0110 0010 0011 0111 (0x6237)
                // Mask 0x0F00: 0000 1111 0000 0000
                // ─────────────────────
                // Result:      0000 0010 0000 0000 = 0x0200 that is not the value expected, we
                // have to shift it 8 bits to the right to ensure that we are getting the real
                // argument:
                // 0x0200 = 0000 0010 0000 0000
                // >> 8   = 0000 0000 0000 0010 = 0x0002
                let register_index = (instruction & 0x0F00) >> 8;
                // Instruction: 0110 0010 0011 0111
                // Mask 0x00FF: 0000 0000 1111 1111
                // ─────────────────────
                // Result:      0000 0000 0011 0111 = 0x0037
                let value = instruction & 0x0FFF;
                println!(
                    "Setting the value {} to the register {}",
                    value, register_index
                );
                registers[register_index as usize] = value;
            }
            JUMP_TO_NNN => {
                // Instruction: 0110 0010 0011 0111
                // Mask 0x0FFF: 0000 1111 1111 1111
                // _______________________________________________________________
                // Result: 0000 0000 0011 0111 = 0x0037
                let value = (instruction & 0x0FFF) as usize;
                // println!("Jumping to the value {}", value);
                program_counter = value;
            }
            DRAW => {
                let register_index_for_x = ((instruction & 0x0F00) >> 8) as usize;
                let register_index_for_y = ((instruction & 0x00F0) >> 4) as usize;

                let number_of_bytes_to_present = (instruction & 0x000F) as usize;
                let x_position = registers[register_index_for_x];
                let y_position = registers[register_index_for_y];
                display.draw(position)
            }
            default => {} // default => panic!("TODO not implemented yet {}", default),
        }
    }
}

fn main() {
    let memory = &mut Memory::new();
    let display = &mut Display::new();
    let result = load_rom("IBM Logo.ch8", memory);
    match result {
        Ok(_) => process_instructions(memory, display),
        Err(e) => panic!("Failed to load the ROM: {}", e),
    }
}
