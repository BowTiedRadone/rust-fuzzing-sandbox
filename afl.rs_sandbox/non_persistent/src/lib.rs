use std::io;

/// System Under Test (SUT) function for fuzzing
///
/// This function contains deliberate vulnerabilities for testing fuzzing
/// capabilities.
pub fn sut(input: &str) -> io::Result<()> {
    // Deliberate out-of-bounds access (unsafe).
    if input.len() >= 100 {
        let bytes = input.as_bytes();
        // Cause an out-of-bounds access.
        let _crash_trigger = bytes[input.len() + 100];
        println!("Crash triggered: {}", _crash_trigger);
    }

    println!("Input content: {}", input);

    Ok(())
}
