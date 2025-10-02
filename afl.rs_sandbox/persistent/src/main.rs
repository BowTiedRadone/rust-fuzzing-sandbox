use afl::fuzz;
use rust_sut::sut;

fn main() {
    fuzz!(|data: &[u8]| {
        // Convert the fuzzer input to a string.
        let content = match std::str::from_utf8(data) {
            Ok(s) => s,
            // Skip invalid UTF-8.
            Err(_) => return,
        };

        // Skip empty inputs.
        if content.trim().is_empty() {
            return;
        }

        // Test the SUT function.
        let _ = sut(content);
    });
}
