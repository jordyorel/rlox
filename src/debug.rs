

use crate::{Chunk, OpCode};


/// Disassembles an entire [`Chunk`], printing each instruction, along with its offset.
///
/// # Arguments
///
/// * `chunk` - Reference to the chunk containing the bytecode.
/// * `name` - A label or identifier for the chunk, shown in the output.
pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

/// When the instruction is a constant instruction, this function prints the 
/// name of the instruction and the value of the constant. 
/// It then returns the offset of the next instruction which is 2 bytes away.
/// Arguments:
/// * `name` - The name of the instruction.
/// * `chunk` - Reference to the chunk containing the bytecode.
/// * `offset` - The position within the chunk's code from which to retrieve the instruction.
/// Returns:
/// The next offset to read from after the current instruction.
pub fn constant_instruction(name: &str, chunk: &Chunk, offset:usize) ->usize {
    if let Some(constant_index) = chunk.get_code( offset + 1) {
        print!("{:<16} {:4} '", name, constant_index);
    
        if let Some(value) = chunk.get_constant(constant_index as usize) {
            println!("{}'", value);
        } else {
            println!("Unknown constant index {}", constant_index);
        }

        offset + 2
    } else {
        println!("Truncade constant instruction at offset {}", offset);
        offset + 1
    }

}

/// Prints the current offset and processes a single instruction from the [`Chunk`].
///
/// # Arguments
///
/// * `chunk` - Reference to the chunk containing the bytecode.
/// * `offset` - The position within the chunk's code from which to retrieve the instruction.
///
/// # Returns
///
/// The next offset to read from after the current instruction.
pub fn disassemble_instruction(chunk: &Chunk, offset:usize) -> usize {
    print!("{:04} ", offset);

    if offset > 0 {
        if let (Some(current_line), Some(prev_line)) = (chunk.get_line(offset), chunk.get_line(offset - 1)) {
            if current_line == prev_line {
                print!("   | ");
            } else {
                print!("{:4} ", current_line);
            }
        }
    } else if let Some(line) = chunk.get_line(offset) {
        print!("{:4} ", line);
    }


    match chunk.get_code(offset).and_then(OpCode::from_u8) {
        Some(OpCode::OpCONSTANT) => {
            constant_instruction("OpCONSTANT", chunk, offset)
        },
        Some(OpCode::OpRETURN) => {
            simple_instructon("OpRETURN", offset)
        },
        _ => {
            println!("Invalid instruction offset");
            offset + 1
        },
    }
}


/// Prints the name of a simple instruction and returns the next offset.
///
/// # Arguments
///
/// * `name` - The name of the instruction.
/// * `offset` - The offset of the instruction within the chunk.
///
/// # Returns
///
/// `offset + 1`, to move to the next instruction.
pub fn simple_instructon(name: &str, offset:usize) -> usize {
    print!("{}\n", name);
    return offset + 1;
}
