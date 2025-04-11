use std::collections::BTreeMap;

pub mod proc;

const MEMORY_SIZE: usize = 65535; // Total memory size

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
    free_blocks: BTreeMap<usize, Vec<MemoryBlock>>, // Map from block size to free blocks
    allocated_blocks: BTreeMap<usize, MemoryBlock>,   // Map from ID to allocated block
    next_id: usize,                                   // Unique ID for allocations
}

impl MemoryManager {
    pub fn new() -> Self {
        let mut free_map = BTreeMap::new();
        free_map.insert(
            MEMORY_SIZE,
            vec![MemoryBlock {
                start: 0,
                size: MEMORY_SIZE,
                allocated: false,
                id: None,
            }],
        );

        Self {
            memory: [0; MEMORY_SIZE],
            free_blocks: free_map,
            allocated_blocks: BTreeMap::new(),
            next_id: 0,
        }
    }

    /// Inserts data into memory using a best-fit allocation strategy.
    /// Returns a unique ID if the allocation is successful.
    pub fn insert(&mut self, size: usize, data: &[u8]) -> Option<usize> {
        // Find the smallest free block (using BTreeMap range) that fits the requested size.
        let mut chosen_key = None;
        let mut chosen_index = None;

        // Iterate over free block sizes starting from `size`
        for (&free_size, blocks) in self.free_blocks.range_mut(size..) {
            if let Some((index, block)) = blocks.iter().enumerate().find(|(_, block)| block.size >= size) {
                chosen_key = Some(free_size);
                chosen_index = Some(index);
                break;
            }
        }

        if let (Some(key), Some(index)) = (chosen_key, chosen_index) {
            // Remove the chosen block from free_blocks.
            let block = {
                let blocks = self.free_blocks.get_mut(&key).unwrap();
                blocks.remove(index)
            };
            // Clean up the entry if no more blocks exist for that key.
            if let Some(blocks) = self.free_blocks.get(&key) {
                if blocks.is_empty() {
                    self.free_blocks.remove(&key);
                }
            }

            // Allocate and write data into memory.
            let new_id = self.next_id;
            self.next_id += 1;

            // Ensure we copy only up to 'size' bytes.
            self.memory[block.start..block.start + size]
                .copy_from_slice(&data[..size]);

            // Store the allocated block.
            let allocated_block = MemoryBlock {
                start: block.start,
                size,
                allocated: true,
                id: Some(new_id),
            };
            self.allocated_blocks.insert(new_id, allocated_block);

            // If there is leftover memory in the free block, add it back to free_blocks.
            if block.size > size {
                let leftover_block = MemoryBlock {
                    start: block.start + size,
                    size: block.size - size,
                    allocated: false,
                    id: None,
                };
                self.free_blocks
                    .entry(leftover_block.size)
                    .or_insert_with(Vec::new)
                    .push(leftover_block);
            }

            return Some(new_id);
        }
        None
    }

    /// Frees an allocated block by its unique ID.
    fn delete(&mut self, id: usize) {
        if let Some(block) = self.allocated_blocks.remove(&id) {
            // Create a free block from the allocated block.
            let free_block = MemoryBlock {
                start: block.start,
                size: block.size,
                allocated: false,
                id: None,
            };
            self.free_blocks
                .entry(free_block.size)
                .or_insert_with(Vec::new)
                .push(free_block);
            println!("Deleted ID: {}", id);
        } else {
            println!("Error: ID {} not found", id);
        }
    }

    /// Returns a slice of the data stored for the allocated block with the given ID.
    fn find(&self, id: usize) -> Option<&[u8]> {
        self.allocated_blocks.get(&id).map(|block| {
            &self.memory[block.start..block.start + block.size]
        })
    }

    /// Reads and prints the allocated block's data by its unique ID.
    fn read(&self, id: usize) {
        match self.allocated_blocks.get(&id) {
            Some(block) => {
                let data = &self.memory[block.start..block.start + block.size];
                println!("Data at ID {}: {:?}", id, data);
            },
            None => println!("Error: ID {} not found", id),
        }
    }

    /// Updates the allocated block's data if the new data does not exceed the block size.
    fn update(&mut self, id: usize, new_data: &[u8]) {
        if let Some(block) = self.allocated_blocks.get_mut(&id) {
            if new_data.len() <= block.size {
                self.memory[block.start..block.start + new_data.len()]
                    .copy_from_slice(new_data);
                println!("Updated ID: {} with new data {:?}", id, new_data);
            } else {
                println!("Error: New data exceeds allocated block size");
            }
        } else {
            println!("Error: ID {} not found", id);
        }
    }

    /// Dumps current free and allocated memory blocks.
    fn dump(&self) {
        println!("Memory Dump:");
        for (size, blocks) in &self.free_blocks {
            for block in blocks {
                println!("FREE: Start: {:#06x}, Size: {}", block.start, size);
            }
        }
        for (id, block) in &self.allocated_blocks {
            println!("ALLOCATED: ID: {}, Start: {:#06x}, Size: {}", id, block.start, block.size);
        }
    }
}

/// Returns the smallest power of two that is greater than or equal to the requested size.
///
/// # Arguments
///
/// * `request` - The size requested to be allocated.
///
/// # Returns
///
/// The smallest power of two that will fit the requested size.
fn next_largest(request: usize) -> usize {
    let mut power = 1;
    while power < request {
        power *= 2;
    }
    power
}

