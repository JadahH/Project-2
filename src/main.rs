use memory_manager::MemoryManager;
use memory_manager::proc::proc::process_file;

/// """Main entry point of the application.
///
/// This function creates a new instance of `MemoryManager`, defines the file path for the command file,
/// and calls `process_file` to process the file. If an error occurs during processing, the error is printed
/// to the standard error stream.
///
/// Note:
///     The function `process_file` is assumed to be defined elsewhere and is responsible for file processing.
///
/// Returns:
///     () -- This function does not return a value. It exits after execution.
/// """


fn main() {
    let mut memory_manager = MemoryManager::new();
    let file_path = "commands.cmmd";
    if let Err(err) = process_file(file_path, &mut memory_manager) {
        eprintln!("Error processing file: {}", err);
    }
}

/*
fn insert(&mut self, size: usize, data: &[u8]) -> Option<usize> {
    let best_fit = self.free_blocks
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
                self.free_blocks.entry(block.size - size).or_insert_with(Vec::new).push(remaining_block);
            }

            return Some(new_id);
        }
    }
    None
} */
