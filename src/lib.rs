use std::collections::BTreeMap;

pub mod proc;

/// """Total size of the managed memory.
///
/// This constant defines the overall number of bytes available.
const MEMORY_SIZE: usize = 65535; // Total memory size

/// """Represents a block of memory managed by the MemoryManager.
///
/// Attributes:
///     start (usize): The starting index of the memory block.
///     size (usize): The size of the memory block in bytes.
///     allocated (bool): Flag indicating if the block is currently allocated.
///     id (Option<usize>): The unique identifier for the allocated block, if any.
#[derive(Debug)]
#[allow(dead_code)]
struct MemoryBlock {
    start: usize,
    size: usize,
    allocated: bool,
    id: Option<usize>,
}

/// """Manages dynamic memory allocation using a best-fit strategy.
///
/// This structure maintains a fixed-size memory region, tracks free blocks in a BTreeMap keyed by block sizes,
/// and tracks allocated blocks by their unique IDs. It supports inserting data, deletion of allocated blocks,
/// reading, updating, and dumping the current memory state.
///
/// Attributes:
///     memory ([u8; MEMORY_SIZE]): The underlying memory array.
///     free_blocks (BTreeMap<usize, Vec<MemoryBlock>>): Maps block sizes to lists of free memory blocks.
///     allocated_blocks (BTreeMap<usize, MemoryBlock>): Maps unique allocation IDs to their corresponding allocated blocks.
///     next_id (usize): Next unique identifier for allocation.
pub struct MemoryManager {
    memory: [u8; MEMORY_SIZE],
    free_blocks: BTreeMap<usize, Vec<MemoryBlock>>, // Map from block size to free blocks
    allocated_blocks: BTreeMap<usize, MemoryBlock>,   // Map from ID to allocated block
    next_id: usize,                                   // Unique ID for allocations
}

impl MemoryManager {
    /// """Creates a new MemoryManager instance with the entire memory available as a single free block.
    ///
    /// Returns:
    ///     MemoryManager: A new instance with initialized memory and free block tracking.
    /// """
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

    /// """Inserts data into memory using a best-fit allocation strategy.
    ///
    /// This method searches for the smallest free memory block that can accommodate the requested size.
    /// If a suitable block is found, it allocates the block, writes the data into memory,
    /// and adjusts free block tracking accordingly.
    ///
    /// Args:
    ///     size (usize): The number of bytes to allocate.
    ///     data (&[u8]): A byte slice containing the data to be stored.
    ///
    /// Returns:
    ///     Option<usize>: A unique allocation ID if the allocation is successful, or None if insufficient space is available.
    /// """
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

    /// """Frees an allocated memory block by its unique ID.
    ///
    /// This method removes the allocated block from the tracking map and re-adds it as a free block.
    /// It prints an appropriate message based on whether the ID was found.
    ///
    /// Args:
    ///     id (usize): The unique allocation ID of the block to be freed.
    ///
    /// Returns:
    ///     None
    /// """
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

    /// """Finds the data associated with an allocated block by its unique ID.
    ///
    /// Args:
    ///     id (usize): The unique allocation ID to look up.
    ///
    /// Returns:
    ///     Option<&[u8]>: A slice of the data stored in the allocated block if found, or None otherwise.
    /// """
    fn find(&self, id: usize) -> Option<&[u8]> {
        self.allocated_blocks.get(&id).map(|block| {
            &self.memory[block.start..block.start + block.size]
        })
    }

    /// """Reads and prints the data of an allocated block identified by its unique ID.
    ///
    /// This method attempts to locate the allocated block and, if found, prints its data; otherwise,
    /// it prints an error message.
    ///
    /// Args:
    ///     id (usize): The unique allocation ID whose data should be printed.
    ///
    /// Returns:
    ///     None
    /// """
    fn read(&self, id: usize) {
        match self.allocated_blocks.get(&id) {
            Some(block) => {
                let data = &self.memory[block.start..block.start + block.size];
                println!("Data at ID {}: {:?}", id, data);
            },
            None => println!("Error: ID {} not found", id),
        }
    }

    /// """Updates the data stored in an allocated block if the new data fits within the block.
    ///
    /// The update occurs only if the length of the new data does not exceed the current allocated block size.
    ///
    /// Args:
    ///     id (usize): The unique allocation ID of the block to update.
    ///     new_data (&[u8]): A byte slice containing the new data.
    ///
    /// Returns:
    ///     None
    /// """
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

    /// """Dumps the current state of memory, listing free and allocated blocks.
    ///
    /// This method prints all free blocks with their starting addresses and sizes,
    /// followed by details of the currently allocated blocks.
    ///
    /// Returns:
    ///     None
    /// """
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

/// """Calculates the smallest power of two that is greater than or equal to a given request size.
///
/// This helper function is useful when ensuring that memory allocations are aligned or sized
/// to the nearest power of two.
///
/// Args:
///     request (usize): The requested allocation size.
///
/// Returns:
///     usize: The smallest power of two that is greater than or equal to the request.
fn next_largest(request: usize) -> usize {
    let mut power = 1;
    while power < request {
        power *= 2;
    }
    power
}
