


/// Enum representing the possible bytecode operations.
///
/// This enum defines the operations supported in the bytecode, 
/// each corresponding to a unique byte value. The values represent
/// the different operations that can be performed in the virtual machine.
pub enum OpCode {
    OpRETURN = 0,
    OpADD = 1,
    OpSUBTRACT = 2,
}

impl OpCode {
    /// Converts a byte value into an `OpCode` variant.
    ///
    /// This function attempts to map a byte value to its corresponding
    /// `OpCode` variant. If the byte does not correspond to any known
    /// operation, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte value to convert into an `OpCode`.
    ///
    /// # Returns
    ///
    /// * `Some(OpCode)` - The corresponding `OpCode` variant if found.
    /// * `None` - If no matching `OpCode` exists for the byte value.
    pub fn from_u8(byte: u8) -> Option<OpCode> {
        match byte {
            0 => Some(OpCode::OpRETURN),
            1 => Some(OpCode::OpADD),
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
///
/// * `count` - The number of instructions currently in the chunk.
/// * `code` - A vector of bytes representing the bytecode instructions.
pub struct Chunk {
    pub count: usize,
    pub code: Vec<u8>,
}

impl Chunk {
    /// Creates a new, empty `Chunk` instance.
    ///
    /// This method initializes the chunk with zero instructions and an empty
    /// code vector, ready to accept new bytecode instructions.
    ///
    /// # Returns
    ///
    /// * A new `Chunk` instance with an empty code vector and count set to zero.
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new(),
        }
    }

    /// Writes a byte (instruction) into the chunk.
    ///
    /// This method appends a new byte to the `code` vector and increments
    /// the `count` to reflect the new instruction.
    ///
    /// # Arguments
    ///
    /// * `byte` - The byte value (instruction) to write to the chunk.
    ///
    /// # Example
    ///
    /// ```
    /// use rlox::Chunk;
    /// let mut chunk = Chunk::new();
    /// chunk.write_chunk(0x01);
    /// ```
    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
        self.count += 1;
    }
}

mod tests {
    
    use crate::Chunk;
    #[test]
    fn test_chunk_write() {
        let mut chunk = Chunk::new();
        chunk.write_chunk(0x01);
        chunk.write_chunk(0x02);

        assert_eq!(chunk.count, 2);
        assert_eq!(chunk.code, vec![0x01, 0x02]);
    }
}

   