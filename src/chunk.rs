
use crate::value::{ValueArray, Value};


/// Enum representing the possible bytecode operations.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OpCode {
    OpRETURN = 0,
    OpCONSTANT = 1,
}

impl OpCode {
    /// Converts a byte value into an `OpCode` variant.
    /// This function attempts to map a byte value to its corresponding
    /// `OpCode` variant. If the byte does not correspond to any known
    /// operation, it returns `None`.
    pub fn from_u8(byte: u8) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::OpRETURN),
            1 => Some(OpCode::OpCONSTANT),
            _ => None,
        }
    }
}

/// Represents a chunk of bytecode in the virtual machine.
///
/// The `Chunk` struct stores a collection of bytecode instructions
/// and provides methods for writing new instructions into the chunk.
///
/// # Fields
/// * `code` - A vector of bytes representing the bytecode instructions.
/// # `lines`- A vector of line numbers corresponding to the instructions.
/// * `constants` - A `ValueArray` containing the constants used in the chunk.
#[derive(Debug)]
pub struct Chunk {
    code: Vec<u8>,
    lines: Vec<usize>,
    constants: ValueArray,
}

impl Chunk {
    /// Creates a new, empty `Chunk` instance.
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            lines: Vec::new(),
            constants: ValueArray::new(),
        }
    }

    /// Writes a byte (instruction) into the chunk.
    /// This method appends a new byte to the `code` vector 
    /// # Arguments
    /// * `byte` - The byte value (instruction) to write to the chunk.
    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Adds a constant value to the chunk's constant pool.
    /// param value - The constant value to add to the chunk.
    /// return - The index of the constant in the constant pool.
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.write_value_array(value);
        self.constants.len() - 1
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub fn get_line(&self, offset: usize) -> Option<usize> {
        self.lines.get(offset).copied()
    }

    pub fn get_code(&self, offset: usize) -> Option<u8> {
        self.code.get(offset).copied()
    }

    pub fn get_constant(&self, index: usize) -> Option<Value> {
        self.constants.get(index)
    }
}

// mod tests {
    
//     use crate::Chunk;
//     #[test]
//     fn test_chunk_write() {
//         let mut chunk = Chunk::new();
//         chunk.write_chunk(0x01, 1);
//         chunk.write_chunk(0x02, 1);

//         assert_eq!(chunk.count, 2);
//         assert_eq!(chunk.code, vec![0x01, 0x02]);
//     }
// }

   