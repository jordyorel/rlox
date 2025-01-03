
use rlox::Chunk;
fn main() {
    let mut chunk = Chunk::new();

    for i in 0..255 {
        chunk.write_chunk(i); // Example bytecode
    }

    // for i in 0..chunk.count {
    //     println!("Chunk contains: {:?}", chunk.code[i]);
    // }

    println!("Chunk count: {}", chunk.count);
    println!("Code length: {}", chunk.code.len());
    println!("Code capcity: {}", chunk.code.capacity());
}
