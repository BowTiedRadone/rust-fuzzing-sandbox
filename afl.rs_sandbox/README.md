# AFL++ Fuzzing Sandbox

This sandbox demonstrates fuzzing Rust code using AFL++ with two different approaches: persistent and non-persistent fuzzing. Both examples target a function with a deliberate out-of-bounds access vulnerability.

## Overview

- **`non_persistent/`** - Traditional file-based fuzzing approach
- **`persistent/`** - In-memory persistent fuzzing approach

## Quick Start

**Setup AFL++** (one-time setup):

```bash
# Build AFL LLVM runtime for your Rust version.
cargo afl config --build

# Configure system for fuzzing (requires root).
cargo afl system-config
```

**Choose your approach**:

- For **non-persistent fuzzing**: See [`non_persistent/README.md`](non_persistent/README.md)
- For **persistent fuzzing**: See [`persistent/README.md`](persistent/README.md)

## Understanding Fuzzing Results

### Crash Analysis

When AFL++ finds crashes, they're stored in `<output-dir>/default/crashes/`:

```bash
# List all crashes.
ls <output-dir>/default/crashes/

# Example crash file.
id:000000,sig:06,src:000001,time:785,execs:2493,op:havoc,rep:27
```

**Crash file naming format:**

- `id:000000` - Unique crash identifier
- `sig:06` - Signal number (6 = SIGABRT)
- `src:000001` - Source test case
- `time:785` - Time to crash (ms)
- `execs:2493` - Executions before crash
- `op:havoc` - Mutation operation that caused crash
- `rep:27` - Repetition count

### Reproducing Crashes

**Non-persistent approach:**

```bash
./target/release/rust-sut out/default/crashes/id:000000,sig:06,src:000001,time:785,execs:2493,op:havoc,rep:27
```

**Persistent approach:**

```bash
cargo afl run out/default/crashes/id:000000,sig:06,src:000001,time:785,execs:2493,op:havoc,rep:27
```

### Fuzzing Statistics

Monitor fuzzing progress in the AFL++ interface:

- **exec/sec** - Executions per second
- **cycles done** - Number of complete queue cycles
- **uniq crashes** - Unique crashes found
- **uniq hangs** - Unique hangs found

### Performance Tips

1. **Use multiple cores**: AFL++ automatically suggests parallel fuzzing
2. **Monitor CPU usage**: Ensure high CPU utilization
3. **Check system limits**: Verify core dumps are enabled
4. **Use release builds**: Significantly faster execution

## Common Issues

### Performance Issues

- Use release builds (`cargo afl build --release`)
- Ensure adequate system resources
- Consider using tmpfs for output directory

## Next Steps

- **Analyze crashes** with debuggers (gdb, lldb)
- **Minimize test cases** using `afl-tmin`
- **Add more seed inputs** to improve coverage
- **Use dictionaries** for structured input formats

---

For detailed setup and usage instructions, see the specific READMEs:

- [Non-Persistent Fuzzing](non_persistent/README.md)
- [Persistent Fuzzing](persistent/README.md)
