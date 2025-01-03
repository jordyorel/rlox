use std::vec::Vec;


pub enum OpCode {
    OpRETURN
}

pub struct Chunk {
    pub count: usize,
    pub code: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            count: 0,
            code: Vec::new()
        }
    }

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