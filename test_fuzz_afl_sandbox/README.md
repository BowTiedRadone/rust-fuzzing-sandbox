# `test-fuzz` sandbox

This is a sandbox for trying out `cargo test-fuzz` – a tool that speeds up the process of building fuzz targets for `afl.rs`.

## How to use it

### 1. Add a target function

One demo target function named `add` can be found inside [main.rs](./src/main.rs). The `test_fuzz` attribute above the function declaration automatically sets up a fuzz target compatible with `afl.rs`.

### 2. Generate corpus

To create a `corpus` or expand an existing one with more entries for each fuzz target, run:

```sh
cargo test
```

After running `cargo test`, the corpus for the `add` target function can be found at `target/corpus/test_fuzz_afl::add/`.

### 3. Start fuzzing

The previously generated corpus will be used to speed up the bug finding process. To start fuzzing the `add` fuzz target, run:

```sh
cargo test-fuzz add
```

The findings for the `add` fuzz target will be found at `target/afl/output/test_fuzz_afl::add/default/`. There, you can see `crashes`, `hangs`, and other important data after fuzzing runs.

### 4. Reproduce a crash

To reproduce an `add` crash found by `afl.rs`, run:

```sh
cargo test-fuzz add --replay crashes
```

## Official Repo

https://github.com/trailofbits/test-fuzz