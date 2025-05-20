# `fuzzcheck` Sandbox

## How to use it

### 1. Use Rust nightly toolchain
```sh
rustup toolchain install nightly
rustup override set nightly
```

### 2. Install `cargo-fuzzcheck`
```sh
cargo install cargo-fuzzcheck
```

### 3. Run `fuzzcheck`
```sh
cargo fuzzcheck tests::test_function_shouldn_t_crash
```

### 4. Check findings
If `fuzzcheck` finds edge cases, they can be checked at `fuzz/tests::test_function_shouldn_t_crash/artifacts/`.

## Official Repo
https://github.com/loiclec/fuzzcheck-rs

