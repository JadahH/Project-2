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

