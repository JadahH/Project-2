use std::collections::BTreeMap;

pub mod proc;

const MEMORY_SIZE: usize = 65535; // Define total memory size

#[derive(Debug)]
#[allow(dead_code)]
struct MemoryBlock {
    start: usize,
    size: usize,
    allocated: bool,
    id: Option<usize>,
}

pub struct MemoryManager {
    memory: [u8; MEMORY_SIZE],
    free_blocks: Vec<MemoryBlock>,      // Best-fit allocation
    allocated_blocks: Vec<MemoryBlock>, // Tracks allocated blocks
    next_id: usize,                     // Unique ID for allocations
}

impl MemoryManager {
    pub fn new() -> Self {
        let free_blocks = vec![MemoryBlock {
            start: 0,
            size: MEMORY_SIZE,
            allocated: false,
            id: None,
        }];

        Self {
            memory: [0; MEMORY_SIZE],
            free_blocks,
            allocated_blocks: vec![],
            next_id: 0,
        }
    }

    fn insert(&mut self, size: usize, data: &[u8]) -> Option<usize> {
        let best_fit = self
            .free_blocks
            .iter_mut()
            .filter(|(block_size, _)| **block_size >= size)
            .min_by_key(|(block_size, _)| **block_size);

        if let Some((&block_size, blocks)) = best_fit {
            if let Some(block) = blocks.pop() {
                if blocks.is_empty() {
                    self.free_blocks.remove(&block_size);
                }

                let new_id = self.next_id;
                self.next_id += 1;

                self.memory[block.start..block.start + size].copy_from_slice(&data);

                let allocated_block = MemoryBlock {
                    start: block.start,
                    size,
                    allocated: true,
                    id: Some(new_id),
                };
                self.allocated_blocks.insert(new_id, allocated_block);

                // Handle leftover memory in the block
                if block.size > size {
                    let remaining_block = MemoryBlock {
                        start: block.start + size,
                        size: block.size - size,
                        allocated: false,
                        id: None,
                    };
                    self.free_blocks
                        .entry(block.size - size)
                        .or_insert_with(Vec::new)
                        .push(remaining_block);
                }

                return Some(new_id);
            }
        }
        None
    }

    fn delete(&mut self, id: usize) {
        todo!("Not Implemented")
    }

    fn find(&self, id: usize) -> Option<&[u8]> {
        todo!("Not Implemented")
    }

    fn read(&self, id: usize) {
        todo!("Not Implemented")
    }

    fn update(&mut self, id: usize, new_data: &[u8]) {
        todo!("Not Implemented")
    }

    fn dump(&self) {
        todo!("Not Implemented")
    }

    /// The user requests a block of a given size `request` and the function will return the
    /// smallest power of 2 that is larger than the requested size.
    ///
    /// # Params:
    ///
    /// - request: The size requested to be allocated
    ///
    /// # Returns
    ///
    /// The smallest power of two that will fit the requested size
    fn next_largest(request: usize) -> usize {}
}
