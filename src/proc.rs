// memory_manager.rs or mod.rs (if you place this in a folder named memory_manager)

use crate::MemoryManager;
use std::collections::BTreeMap;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;

/// """Module containing process-related functions for the memory manager.
///
/// This module defines functions to process command files which control memory allocation
/// and related operations, such as INSERT, DELETE, FIND, READ, UPDATE, and DUMP.
pub mod proc {
    use super::MemoryManager;
    use std::io::{self, BufRead};
    use std::fs::File;
    use std::path::Path;

    /// """Processes a file containing commands to manipulate the memory manager.
    ///
    /// The supported commands are: INSERT, DELETE, FIND, READ, UPDATE, and DUMP.
    ///
    /// Args:
    ///     file_path (str): The path to the command file.
    ///     memory_manager (MemoryManager): A mutable reference to the memory manager instance.
    ///
    /// Returns:
    ///     io::Result<()>: Ok(()) if processing was successful; otherwise, an I/O error.
    /// """
    pub fn process_file(file_path: &str, memory_manager: &mut MemoryManager) -> io::Result<()> {
        if let Ok(lines) = read_lines(file_path) {
            for line in lines.flatten() {
                println!("Processing line: {}", line);
                let tokens: Vec<&str> = line.split_whitespace().collect();
                if tokens.is_empty() {
                    continue;
                }
                match tokens[0] {
                    "INSERT" => {
                        if tokens.len() < 3 {
                            println!("Error: Invalid INSERT command");
                            continue;
                        }
                        // Parse the size and use the third token as data (as bytes).
                        if let (Ok(size), data) = (tokens[1].parse::<usize>(), tokens[2].as_bytes()) {
                            if let Some(id) = memory_manager.insert(size, data) {
                                println!("Allocated ID: {}", id);
                            } else {
                                println!("Memory allocation failed");
                            }
                        }
                    }
                    "DELETE" => {
                        if tokens.len() < 2 {
                            println!("Error: Invalid DELETE command");
                            continue;
                        }
                        if let Ok(id) = tokens[1].parse::<usize>() {
                            memory_manager.delete(id);
                        }
                    }
                    "FIND" => {
                        if tokens.len() < 2 {
                            println!("Error: Invalid FIND command");
                            continue;
                        }
                        if let Ok(id) = tokens[1].parse::<usize>() {
                            if let Some(data) = memory_manager.find(id) {
                                println!("Data at {}: {:?}", id, data);
                            } else {
                                println!("Nothing at {}", id);
                            }
                        }
                    }
                    "READ" => {
                        if tokens.len() == 2 {
                            if let Ok(id) = tokens[1].parse::<usize>() {
                                memory_manager.read(id);
                            } else {
                                println!("Invalid READ command format");
                            }
                        }
                    }
                    "UPDATE" => {
                        if tokens.len() < 3 {
                            println!("Error: Invalid UPDATE command");
                            continue;
                        }
                        if let (Ok(id), new_data) = (tokens[1].parse::<usize>(), tokens[2].as_bytes()) {
                            memory_manager.update(id, new_data);
                        }
                    }
                    "DUMP" => {
                        memory_manager.dump();
                    }
                    _ => {
                        println!("Error: Unknown command `{}`", tokens[0]);
                    }
                }
            }
        }
        Ok(())
    }

    /// """Reads lines from a file.
    ///
    /// Args:
    ///     filename: A value that can be referenced as a file path.
    ///
    /// Returns:
    ///     io::Result<io::Lines<io::BufReader<File>>>: An iterator over the lines in the file or an I/O error.
    /// """
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}

/// """Represents a memory block used to track allocated or free space.
///
/// Attributes:
///     start (usize): The starting index of the block.
///     size (usize): The size of the block in bytes.
///     allocated (bool): Whether the block is currently allocated.
///     id (Option<usize>): The unique identifier for an allocated block, if applicable.
#[derive(Debug)]
#[allow(dead_code)]
pub struct MemoryBlock {
    pub start: usize,
    pub size: usize,
    pub allocated: bool,
    pub id: Option<usize>,
}

/// """Manages a fixed-size memory region with simple allocation and deallocation strategies.
///
/// This structure uses a best-fit strategy to allocate memory and tracks free and allocated blocks
/// using BTreeMaps.
pub struct MemoryManager {
    pub memory: [u8; 65535],
    pub free_blocks: BTreeMap<usize, Vec<MemoryBlock>>,
    pub allocated_blocks: BTreeMap<usize, MemoryBlock>,
    pub next_id: usize,
}

impl MemoryManager {
    /// """Creates a new memory manager with one large free block covering all available memory.
    ///
    /// Returns:
    ///     MemoryManager: A new instance of MemoryManager.
    /// """
    pub fn new() -> Self {
        let mut free_blocks = BTreeMap::new();
        free_blocks.insert(
            65535,
            vec![MemoryBlock {
                start: 0,
                size: 65535,
                allocated: false,
                id: None,
            }],
        );
        Self {
            memory: [0; 65535],
            free_blocks,
            allocated_blocks: BTreeMap::new(),
            next_id: 0,
        }
    }

    /// """Attempts to insert data into memory using a best-fit free block.
    ///
    /// Finds a free block of sufficient size, writes the data into memory, adjusts free block listings,
    /// and returns a unique identifier for the allocation.
    ///
    /// Args:
    ///     size (usize): The number of bytes to allocate.
    ///     data (&[u8]): A byte slice containing data to store.
    ///
    /// Returns:
    ///     Option<usize>: Some(unique ID) if allocation is successful; None otherwise.
    /// """
    pub fn insert(&mut self, size: usize, data: &[u8]) -> Option<usize> {
        // Find a free block whose size is at least `size`.
        let mut chosen_key = None;
        let mut chosen_index = None;
        for (&free_size, blocks) in self.free_blocks.range_mut(size..) {
            if let Some((index, block)) = blocks.iter().enumerate().find(|(_, block)| block.size >= size) {
                chosen_key = Some(free_size);
                chosen_index = Some(index);
                break;
            }
        }
        if let (Some(key), Some(index)) = (chosen_key, chosen_index) {
            // Remove the chosen free block.
            let block = {
                let blocks = self.free_blocks.get_mut(&key).unwrap();
                blocks.remove(index)
            };
            // Clean up the key if its vector is empty.
            if let Some(blocks) = self.free_blocks.get(&key) {
                if blocks.is_empty() {
                    self.free_blocks.remove(&key);
                }
            }
            let new_id = self.next_id;
            self.next_id += 1;
            // Write the data into memory.
            self.memory[block.start..block.start + size].copy_from_slice(&data[..size]);
            let allocated_block = MemoryBlock {
                start: block.start,
                size,
                allocated: true,
                id: Some(new_id),
            };
            self.allocated_blocks.insert(new_id, allocated_block);
            // If there is leftover space, add it back into free blocks.
            if block.size > size {
                let leftover = MemoryBlock {
                    start: block.start + size,
                    size: block.size - size,
                    allocated: false,
                    id: None,
                };
                self.free_blocks
                    .entry(leftover.size)
                    .or_insert_with(Vec::new)
                    .push(leftover);
            }
            return Some(new_id);
        }
        None
    }

    /// """Deletes an allocated block by its unique ID and returns the freed block to free memory.
    ///
    /// Args:
    ///     id (usize): The unique identifier of the block to delete.
    ///
    /// Returns:
    ///     None
    /// """
    pub fn delete(&mut self, id: usize) {
        if let Some(block) = self.allocated_blocks.remove(&id) {
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

    /// """Finds a block by its unique ID and returns a slice of its data.
    ///
    /// Args:
    ///     id (usize): The unique identifier of the allocated block.
    ///
    /// Returns:
    ///     Option<&[u8]>: A slice of data from the allocated block if it exists; None otherwise.
    /// """
    pub fn find(&self, id: usize) -> Option<&[u8]> {
        self.allocated_blocks.get(&id).map(|block| {
            &self.memory[block.start..block.start + block.size]
        })
    }

    /// """Reads and prints the data stored in the block specified by its unique ID.
    ///
    /// Args:
    ///     id (usize): The unique identifier of the block to read.
    ///
    /// Returns:
    ///     None
    /// """
    pub fn read(&self, id: usize) {
        match self.allocated_blocks.get(&id) {
            Some(block) => {
                let data = &self.memory[block.start..block.start + block.size];
                println!("Data at ID {}: {:?}", id, data);
            }
            None => println!("Error: ID {} not found", id),
        }
    }

    /// """Updates the data for a given block if the new data does not exceed the allocated size.
    ///
    /// Args:
    ///     id (usize): The unique identifier of the block to update.
    ///     new_data (&[u8]): A byte slice containing the new data.
    ///
    /// Returns:
    ///     None
    /// """
    pub fn update(&mut self, id: usize, new_data: &[u8]) {
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

    /// """Dumps a summary of all free and allocated memory blocks.
    ///
    /// Prints the starting address and size of each free block, and details of each allocated block.
    ///
    /// Returns:
    ///     None
    /// """
    pub fn dump(&self) {
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
