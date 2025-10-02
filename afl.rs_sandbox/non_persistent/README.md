# Non-Persistent Fuzzing with AFL++

This project demonstrates non-persistent fuzzing using AFL++ to find vulnerabilities in a Rust function. The target function has a deliberate out-of-bounds access bug that the fuzzer will discover.

## Build

Navigate to the project directory:

```bash
cd afl.rs_sandbox/non_persistent/
```

Build the binary:

```bash
# Debug build (faster compilation).
cargo afl build

# Release build (faster execution).
cargo afl build --release
```

## Run

### Single Core

```bash
# Use 'corpus' directory as the input.
# Store the fuzzing campaign results under an 'out' directory.
# NOTE: '@@' allows AFL to substitute the path to each test case file.
cargo afl fuzz -i corpus -o out target/release/rust-sut @@
```

### Multi-Core (Recommended)

```bash
# Terminal 1 (master).
cargo afl fuzz -i corpus -o out -M main target/release/rust-sut @@

# Terminal 2+ (workers).
cargo afl fuzz -i corpus -o out -S worker1 target/release/rust-sut @@
cargo afl fuzz -i corpus -o out -S worker2 target/release/rust-sut @@
```

## Files

- `corpus/` - Seed input files
- `out/` - AFL++ results (crashes, queue, statistics)

## Features

- Non-persistent mode (file-based input)
- CmpLog level 2 (enabled by default)
- Testcache with 50 MB
- Exploration-based constant power schedule (EXPLORE)
- Release optimizations

## Testing

Test the program manually:

```bash
# View seed input.
cat ./corpus/seed1.txt

# Run with seed file.
./target/release/rust-sut ./corpus/seed1.txt
```

---

Happy fuzzing!
