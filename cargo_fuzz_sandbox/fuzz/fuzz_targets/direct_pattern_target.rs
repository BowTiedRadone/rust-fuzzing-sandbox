#![no_main]

use libfuzzer_sys::fuzz_target;

// Fuzz target that crashes on inputs containing "a815".
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        if s.contains("a815") {
            panic!("Found 'a815' in input: '{s}'");
        }
    }
});