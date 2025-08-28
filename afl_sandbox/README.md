# AFL++ Fuzz Testing with Rust

This repository contains a naive Rust program designed to demonstrate fuzz testing with AFL++. The example program reads input from a file, processes the data, and intentionally includes a potential out-of-bounds error. By using AFL++, we can generate crashes and identify flaws in the code, providing a practical introduction to fuzzing techniques and AFL++ usage.

## Getting Started

### Prerequisites

- [`afl.rs` installed](https://github.com/rust-fuzz/afl.rs)

### Steps

Build the program using `cargo afl`:

```bash
$ cd afl_sandbox
$ cargo afl build
```

Create an input(corpus) directory and add at least one input (seed). The sandbox already has an input directory containing a seed file:

```bash
$ ls corpus/
seed1.txt
```

Test the test cases from the `corpus` directory manually:

```bash
$ cat ./corpus/test.txt
test

$ ./target/debug/afl-rust ./corpus/test.txt
Buffer content: test
```

Start fuzzing the binary:

```bash
# Use 'corpus' directory as the input.
# Store the fuzzing campaign results under an 'out' directory.
# NOTE: '@@' allows AFL to substitute the path to each test case file.
$ cargo afl fuzz -i corpus -o out target/debug/afl-rust @@
```

Reproduce eventual crashes stored under `out/default/crashes`:

```bash
$ ls out/default/crashes/
id:000000,sig:06,src:000001,time:785,execs:2493,op:havoc,rep:27  README.txt

$ cargo afl run out/default/crashes/id\:000000\,sig\:06\,src\:000001\,time\:785\,execs\:2493\,op\:havoc\,rep\:27

$ ./target/debug/afl-rust out/default/crashes/id:000000,sig:06,src:000000,time:12345,execs:67890
```

---

Happy fuzzing!
