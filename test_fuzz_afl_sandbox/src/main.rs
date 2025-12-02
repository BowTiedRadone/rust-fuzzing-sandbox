pub fn main() {
    println!("Hello, world!");
}

#[test_fuzz::test_fuzz]
pub fn add(left: u64, right: u64) -> u64 {
    // Early return if the sum would overflow.
    if left > u64::MAX - right {
        println!("Overflow!");
        return 0;
    }
    println!("left: {}, right: {}", left, right);
    // This is a test to see if the fuzzer can catch this condition.
    // If the fuzzer finds a way to get left == 1234 && right == 4321, then it
    // will panic and the fuzzer should report a crash.
    if left == 1234 && right == 4321 {
        panic!("Fuzzing caught this!");
    }
    left + right
}
