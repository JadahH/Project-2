use memory_manager::{proc::process_file, MemoryManager};

fn main() {
    let mut memory_manager = MemoryManager::new();
    let file_path = "commands.cmmd";
    if let Err(err) = process_file(file_path, &mut memory_manager) {
        eprintln!("Error processing file: {}", err);
    }
}
