# Cargo Fuzz Sandbox

This is a sandbox for trying out `cargo fuzz` - a tool that helps find bugs in Rust code by automatically testing it with random inputs.

## How to use it

### 1. Add a new fuzz target

```bash
cargo fuzz add <target-name>
```
This creates a new fuzz testing target with the name you choose.

### 2. See what fuzz targets exist

```bash
cargo fuzz list
```
This shows all the fuzz targets available in the project.

### 3. Run a fuzz test

```bash
cargo fuzz run <target-name>
```

Or if you prefer not to change your default Rust toolchain:

```bash
cargo +nightly fuzz run <target-name>
```

**Note:** Cargo fuzz needs the nightly Rust toolchain. You can either set nightly as your project default or just use the `+nightly` flag as shown above.

## Available Fuzz Targets

### direct_pattern_target

This target uses a simple approach to find the pattern "a815" in inputs. It directly checks if the input contains the pattern using the `contains()` method. This is a straightforward way to detect specific patterns in fuzzer inputs.

### branched_target

This target has multiple code paths (branches). It's designed to show how fuzz testing can find and explore different parts of your code. The fuzzer will try to generate inputs that reach all these different code paths.

### More to come

Additional fuzz targets will be added to this sandbox in the future.

## Official Docs

https://rust-fuzz.github.io/book/cargo-fuzz.html