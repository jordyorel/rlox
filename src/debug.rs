

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
    while offset < chunk.count {
        offset = disassemble_instruction(chunk, offset);
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
    let instruction = chunk.code[offset];

    match OpCode::from_u8(instruction) {
        Some(OpCode::OpRETURN) => simple_instructon("OpRETURN", offset),
        Some(OpCode::OpADD) => simple_instructon("OpADD", offset),
        _ => {
            println!("Unknown instruction {}", instruction);
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