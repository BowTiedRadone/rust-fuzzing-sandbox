#![no_main]

use cargo_fuzz_sandbox::{hello_basic, hello_testing, test_utils};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Always available function.
    let basic_msg = hello_basic();
    println!("Basic: {}", basic_msg);
    
    // Function only available with testing feature.
    let testing_msg = hello_testing();
    println!("Testing: {}", testing_msg);
    
    // Test utilities only available with testing feature.
    let test_data = test_utils::create_test_data();
    let is_valid = test_utils::validate_test_data(&test_data);
    println!("Test data valid: {}", is_valid);
    
    // Simple fuzzing logic using the input data.
    if !data.is_empty() {
        let sum = data.iter().map(|&b| b as u32).sum::<u32>();
        println!("Data sum: {}", sum);
        
        // Use test utilities to process the fuzz input.
        if data.len() >= 5 {
            let fuzz_slice = &data[..5].iter().map(|&b| b as i32).collect::<Vec<_>>();
            let is_valid_fuzz = test_utils::validate_test_data(fuzz_slice);
            println!("Fuzz data validation: {}", is_valid_fuzz);
        }
    }
});
