use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::MemoryManager;

pub fn process_file(file_path: &str, memory_manager: &mut MemoryManager) -> io::Result<()> {
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            println!("Processing line: {}", line); // Debug print
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

                "READ" if tokens.len() == 2 => {
                    if let Ok(id) = tokens[1].parse::<usize>() {
                        memory_manager.read(id);
                    } else {
                        println!("Invalid READ command format");
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
