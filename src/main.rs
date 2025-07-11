mod display;
mod keyboard;
mod memory;
mod stack;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use display::{Display, DisplayTrait};
use keyboard::{Keyboard, KeyboardTrait};
use memory::MemoryTrait;
use memory::{MEMORY_SIZE, Memory, SharedMemory};
use rand::Rng;
use stack::{Stack, StackTrait};
use std::cell::RefCell;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;

const PROGRAM_START: usize = 0x200;
const SKIP_NEXT_INSTRUCTION_IF_X_IS_EQUAL_TO_Y: u16 = 0x5000;
const SKIP_NEXT_INSTRUCTION_IF_X_IS_DIFFERENT_OF_Y: u16 = 0x9000;
const SKIP_NEXT_INSTRUCTION_IF_X_IS_EQUAL_TO_KK: u16 = 0x3000;
const SKIP_NEXT_INSTRUCTION_IF_X_IS_DIFFERENT_OF_KK: u16 = 0x4000;
const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START;
const CLEAR: u16 = 0x00E0; // Clear screen (exact match, no mask needed because it doesn't send
const POP_THE_TOP_OF_THE_STACK_AS_THE_CURRENT_PROGRAM_COUNTER: u16 = 0x00EE; // Clear screen (exact match, no mask needed because it doesn't send
// args)
const JUMP_TO_NNN: u16 = 0x1000; // 1NNN: Jump to address NNN 
const JUMP_TO_NNN_PLUS_V0: u16 = 0xB000; // 1NNN: Jump to address NNN 
const RANDOM_AND_AND_WITH_KK: u16 = 0xC000; // 1NNN: Jump to address NNN 
const SET_PROGRAM_COUNTER_TO_THE_TOP_OF_THE_STACK_AND_GO_TO_NNN: u16 = 0x2000; // 1NNN: Jump to address NNN 
const SET_NN_TO_VX: u16 = 0x6000; // 6XNN: Set VX to NN 
const SUM_NN_TO_VX: u16 = 0x7000; // 7XNN: Add NN to VX 
const STORE_THE_VALUE_OF_REGISTER_X_TO_REGISTER_Y: u16 = 0x8000; // 7XNN: Add NN to VX 
const OR_TO_X_AND_Y: u16 = 0x8001; // 7XNN: Add NN to VX 
const XOR_TO_X_AND_Y: u16 = 0x8003; // 7XNN: Add NN to VX 
const AND_TO_X_AND_Y: u16 = 0x8002; // 7XNN: Add NN to VX 
const ADD_VX_WITH_VY_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS: u16 = 0x8004; // 7XNN: Add NN to VX 
const SUBTRACT_VX_WITH_VY_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS: u16 = 0x8005; // 7XNN: Add NN to VX 
const SUBTRACT_VY_WITH_VX_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS: u16 = 0x8007; // 7XNN: Add NN to VX 
const SHIFT_RIGHT_X: u16 = 0x8006; // 7XNN: Add NN to VX 
const SKIP_NEXT_INSTRUCTION_IF_X_KEY_WAS_PRESSED: u16 = 0xE09E; // 7XNN: Add NN to VX 
const SKIP_NEXT_INSTRUCTION_IF_X_KEY_WAS_NOT_PRESSED: u16 = 0xE0A1; // 7XNN: Add NN to VX 
const SET_DELAY_TIMER_TO_VX: u16 = 0xF007; // 7XNN: Add NN to VX 
const WAIT_TO_A_KEY_TO_BE_PRESSED_AND_STORE_IT_ON_THE_VX: u16 = 0xF00A; // 7XNN: Add NN to VX 
const SHIFT_LEFT_X: u16 = 0x800E; // 7XNN: Add NN to VX 
const SET_NNN_TO_I: u16 = 0xA000; // ANNN: Set I to NNN 
const DRAW: u16 = 0xD000; // DXYN: Draw sprite at (VX, VY) with height N 

pub fn load_rom(file_path: &str, memory: SharedMemory) -> Result<Vec<u8>, String> {
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
        memory.borrow_mut().write(PROGRAM_START + i, byte);
    }

    println!("Loaded ROM: {} ({} bytes)", file_path, rom_data.len());
    println!("Memory: {}", memory.borrow());
    Ok(rom_data)
}

fn process_instructions(
    memory: SharedMemory,
    display: &mut Display,
    stack: &mut Stack,
    keyboard: &mut Keyboard,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut program_counter = PROGRAM_START;
    let mut registers: [u8; 16] = [0x00; 16];
    let mut i_register = 0x00;
    let mut f_register: u8 = 0x00;
    let mut program_counter_sum_value = 2;
    loop {
        enable_raw_mode()?;
        keyboard.process_any_input();
        disable_raw_mode()?;
        // display.print();
        let first_byte = memory.borrow().retrieve(program_counter) as u16;
        let second_byte = memory.borrow().retrieve(program_counter + 1) as u16;
        let instruction = (first_byte << 8) + second_byte;
        program_counter = program_counter + program_counter_sum_value;
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
            POP_THE_TOP_OF_THE_STACK_AS_THE_CURRENT_PROGRAM_COUNTER => {
                let value = stack.pop() as usize;
                println!("Stack popping the value {} to program_counter", value);
                program_counter = value;
            }
            SET_PROGRAM_COUNTER_TO_THE_TOP_OF_THE_STACK_AND_GO_TO_NNN => {
                // Instruction: 0110 0010 0011 0111
                // Mask 0x0FFF: 0000 1111 1111 1111
                // _______________________________________________________________
                // Result: 0000 0000 0011 0111 = 0x0037
                let value = (instruction & 0x0FFF) as usize;
                println!(
                    "Stack pushing {} and program_counter navigating to {}",
                    program_counter, value
                );
                stack.push(program_counter as u16);
                program_counter = value;
            }
            SKIP_NEXT_INSTRUCTION_IF_X_IS_EQUAL_TO_KK => {
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                let k_arg = (instruction & 0x00FF) as u8;
                if registers[register_index] == k_arg {
                    program_counter += 2;
                }
            }
            STORE_THE_VALUE_OF_REGISTER_X_TO_REGISTER_Y => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                registers[x_register_index] = registers[y_register_index];
            }
            SKIP_NEXT_INSTRUCTION_IF_X_IS_DIFFERENT_OF_Y => {
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = (instruction & 0x00F0 >> 4) as usize;
                if registers[register_index] != registers[y_register_index] {
                    program_counter += 2;
                }
            }
            SKIP_NEXT_INSTRUCTION_IF_X_IS_DIFFERENT_OF_KK => {
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                let k_arg = (instruction & 0x00FF) as u8;
                if registers[register_index] != k_arg {
                    program_counter += 2;
                }
            }
            AND_TO_X_AND_Y => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                registers[x_register_index] =
                    registers[x_register_index] & registers[y_register_index]
            }
            XOR_TO_X_AND_Y => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                registers[x_register_index] =
                    registers[x_register_index] ^ registers[y_register_index];
            }
            OR_TO_X_AND_Y => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                registers[x_register_index] =
                    registers[x_register_index] | registers[y_register_index]
            }
            SKIP_NEXT_INSTRUCTION_IF_X_KEY_WAS_PRESSED => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let pressed_key = registers[x_register_index];
                if keyboard.is_key_pressed(pressed_key) {
                    program_counter += 2;
                }
            }
            SKIP_NEXT_INSTRUCTION_IF_X_KEY_WAS_NOT_PRESSED => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let pressed_key = registers[x_register_index];
                if !keyboard.is_key_pressed(pressed_key) {
                    program_counter += 2;
                }
            }
            SKIP_NEXT_INSTRUCTION_IF_X_IS_EQUAL_TO_Y => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                if registers[x_register_index] == registers[y_register_index] {
                    program_counter += 2;
                }
            }
            SET_DELAY_TIMER_TO_VX => {
                // TODO implement this when we have the delay timer implemented
            }
            WAIT_TO_A_KEY_TO_BE_PRESSED_AND_STORE_IT_ON_THE_VX => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                registers[x_register_index] = keyboard.get_key_pressed();
            }
            ADD_VX_WITH_VY_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;
                let sum = registers[x_register_index] + registers[y_register_index];

                if sum > 255 {
                    f_register = 1;
                } else {
                    f_register = 0;
                }

                registers[x_register_index] = (sum & 0xFF) as u8; // Store lower 8 bits
            }
            SHIFT_RIGHT_X => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                // 0x01 represents 0000 0001 so we are checking if the last bit is 1
                // this mask only validate the last bit ignoring the rest transforming them in 0
                if registers[x_register_index] & 0x01 == 0x01 {
                    f_register = 1;
                } else {
                    f_register = 0;
                }

                registers[x_register_index] = registers[x_register_index] >> 1;
            }
            SHIFT_LEFT_X => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;

                // 0x80 represents 1000 0000 so we are checking if the first bit is 1
                // this mask only validate the first bit ignoring the rest transforming them in 0
                if registers[x_register_index] & 0x80 == 0x80 {
                    f_register = 1;
                } else {
                    f_register = 0;
                }

                registers[x_register_index] = registers[x_register_index] << 1;
            }
            SUBTRACT_VX_WITH_VY_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;

                if registers[x_register_index] >= registers[y_register_index] {
                    f_register = 1;
                } else {
                    f_register = 0;
                }

                registers[x_register_index] =
                    registers[x_register_index].wrapping_sub(registers[y_register_index]);
            }

            SUBTRACT_VY_WITH_VX_AND_SET_TRUE_TO_VF_IF_ITS_MORE_THAN_8_BITS => {
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let y_register_index = ((instruction & 0x00F0) >> 4) as usize;

                if registers[y_register_index] >= registers[x_register_index] {
                    f_register = 1;
                } else {
                    f_register = 0;
                }

                registers[x_register_index] =
                    registers[y_register_index].wrapping_sub(registers[x_register_index]);
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
                registers[register_index as usize] = registers[register_index] + value as u8;
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
                registers[register_index as usize] = value as u8;
            }
            RANDOM_AND_AND_WITH_KK => {
                // Instruction: 0110 0010 0011 0111
                // Mask 0x0FFF: 0000 1111 1111 1111
                // _______________________________________________________________
                // Result: 0000 0000 0011 0111 = 0x0037
                let mut rng = rand::rng();
                let kk_mask = (instruction & 0x00FF) as u8;
                let x_register_index = ((instruction & 0x0F00) >> 8) as usize;
                let random_byte = rng.random_range(0..256);
                println!(
                    "random number provided: {} (must be between 0 to 256)",
                    random_byte
                );
                registers[x_register_index] = random_byte & kk_mask;
            }
            JUMP_TO_NNN_PLUS_V0 => {
                // Instruction: 0110 0010 0011 0111
                // Mask 0x0FFF: 0000 1111 1111 1111
                // _______________________________________________________________
                // Result: 0000 0000 0011 0111 = 0x0037
                let value = (instruction & 0x0FFF) as usize;
                // println!("Jumping to the value {}", value);
                program_counter = value + registers[0] as usize;
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
                let y = registers[register_index_for_y];
                let x = registers[register_index_for_x];
                display.draw(
                    x as usize,
                    y as usize,
                    number_of_bytes_to_present,
                    i_register,
                );
            }
            default => {} // default => panic!("TODO not implemented yet {}", default),
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stack = &mut Stack::new();
    let memory: SharedMemory = Rc::new(RefCell::new(Memory::new()));
    let display = &mut Display::new(Rc::clone(&memory));
    let result = load_rom("IBM Logo.ch8", Rc::clone(&memory));
    let keyboard = &mut Keyboard::new();
    match result {
        Ok(_) => process_instructions(memory, display, stack, keyboard),
        Err(e) => panic!("Failed to load the ROM: {}", e),
    }?;
    Ok(())
}
