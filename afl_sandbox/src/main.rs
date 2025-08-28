use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Fetch command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if an argument (file name) is provided
    if args.len() < 2 {
        eprintln!("Usage: <input-file>");
        return Ok(());
    }

    // Open the input file
    let file_path = &args[1];
    let mut file = File::open(file_path)?;

    // Read file content into a buffer
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Ensure the buffer is null-terminated
    buffer.push(0);

    // Deliberate out-of-bounds access (unsafe)
    if buffer.len() >= 100 {
        // Introduce a bug: accessing buffer beyond its length
        let _crash_trigger = buffer[buffer.len() + 100]; // Cause an out-of-bounds access
    }

    println!("Buffer content: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}
