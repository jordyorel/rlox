
use rlox::Chunk; 
use rlox::OpCode;
mod debug;


fn main() {
    let mut chunk = Chunk::new();

    chunk.write_chunk(OpCode::OpADD as u8);
    chunk.write_chunk(OpCode::OpRETURN as u8);

    // Disassemble the chunk (first pass)
    debug::disassemble_chunk(&chunk, "test chunk");



}
