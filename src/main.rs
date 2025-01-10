
use rlox::Chunk; 
use rlox::OpCode;
mod debug;


fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.5);

    chunk.write_chunk(OpCode::OpCONSTANT as u8, 123);
    chunk.write_chunk(constant as u8, 123);

    chunk.write_chunk(OpCode::OpRETURN as u8, 124);

    // Disassemble the chunk (first pass)
    debug::disassemble_chunk(&chunk, "test chunk");

}
