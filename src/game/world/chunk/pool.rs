use super::chunk::Chunk;


pub struct ChunkPool {
    available: Vec<Chunk>,
    max_size: usize,
}

impl ChunkPool {
    pub fn new(max_size: usize) -> Self {
        println!("Creating chunk pool with max size {}", max_size);
        Self {
            available: Vec::with_capacity(max_size),
            max_size,
        }
    }

    pub fn get_chunk(&mut self) -> Option<Chunk> {
        self.available.pop()
    }

    pub fn return_chunk(&mut self, mut chunk: Chunk) {
        chunk.tiles.clear();
        chunk.blocks.clear();

        if self.available.len() < self.max_size {
            self.available.push(chunk);
        } else {
            println!("Pool is full, discarding chunk");
        }
    }

    pub fn available_chunks(&self) -> usize {
        self.available.len()
    }

    pub fn is_full(&self) -> bool {
        self.available.len() >= self.max_size
    }
}