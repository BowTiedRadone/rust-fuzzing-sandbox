#![no_main]

use libfuzzer_sys::fuzz_target;

// Branched fuzz target that crashes on inputs containing "a815".
fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        for i in 0..s.len().saturating_sub(3) {
            let bytes = s.as_bytes();
            
            // Check for "a815" pattern.
            if bytes[i] == b'a' {
                if bytes.get(i+1) == Some(&b'8') {
                    if bytes.get(i+2) == Some(&b'1') {
                        if bytes.get(i+3) == Some(&b'5') {
                            panic!("Found a815 at position {i}");
                        }
                    }
                }
            }
        }
    }
});