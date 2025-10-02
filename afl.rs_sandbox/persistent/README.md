# Persistent Fuzzing with AFL++

This project demonstrates persistent-mode fuzzing using AFL++ to find vulnerabilities in a Rust function. The target function has a deliberate out-of-bounds access bug that the fuzzer will discover.

## Build

Navigate to the project directory:

```bash
cd afl.rs_sandbox/persistent/
```

Build the fuzzer:

```bash
# Debug build (faster compilation)
cargo afl build

# Release build (faster execution)
cargo afl build --release
```

## Run

### Single Core

```bash
cargo afl fuzz -i corpus/ -o out -x dict.txt ./target/release/persistent-fuzzer
```

### Multi-Core (Recommended)

```bash
# Terminal 1 (master)
cargo afl fuzz -i corpus/ -o out -M main -x dict.txt ./target/release/persistent-fuzzer

# Terminal 2+ (workers)
cargo afl fuzz -i corpus/ -o out -S worker1 -x dict.txt ./target/release/persistent-fuzzer
cargo afl fuzz -i corpus/ -o out -S worker2 -x dict.txt ./target/release/persistent-fuzzer
```

## Files

- `corpus/` - Seed input files
- `dict.txt` - Input examples and patterns
- `out/` - AFL++ results (crashes, queue, statistics)

## Features

- Persistent mode using `afl::fuzz!` macro
- Deferred forkserver (enabled by default)
- CmpLog enabled by default
- Release optimizations

---

Happy fuzzing!
