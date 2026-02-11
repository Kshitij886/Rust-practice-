# Rust projects

Small collection of Rust example programs and exercises used for learning Rust concepts.

Repository layout
- `Cargo.toml` - workspace manifest
- `src/main.rs` - main entry (may run default example or act as a harness)
- `src/bin/` - example binaries demonstrating individual concepts (one file = one binary)

Notable examples (under `src/bin`)
- `channel.rs` — channel-based concurrency and message passing examples
- `chrono.rs` — examples using `chrono` for date/time handling
- `generics.rs` — demonstrations of generic types and functions
- `hash_iter.rs` — iterating over hash collections
- `hashmap.rs` / `hashmaps.rs` — examples showing `HashMap` usage
- `iter_que.rs`, `iteraters.rs` — iterator usage and patterns
- `lifetimes.rs` / `structs_with_lifetimes.rs` — lifetime annotations and structs
- `message_passing.rs` — threads communicating via channels
- `move_multithreading.rs`, `multithreading.rs` — multithreading and ownership in threads
- `moving.rs` — ownership and move semantics
- `option_enum.rs` — working with `Option` and enums
- `read_content.rs` — reading files / I/O examples
- `slices.rs`, `strings.rs`, `vectors.rs` — common collection and string operations
- `traits.rs`, `traits_as_params.rs` — traits and passing trait objects / generics

Quick start

Prerequisites
- Rust toolchain (rustc + cargo). Install via https://rustup.rs

Run the default binary
```bash
cargo run
```

Run a specific example binary (by file name without `.rs`)
```bash
cargo run --bin channel
cargo run --bin message_passing
cargo run --bin lifetimes
```

Build only
```bash
cargo build --release
```

Add a new example
1. Create `src/bin/your_example.rs`.
2. Make it `fn main()` and include your example code.
3. Run with `cargo run --bin your_example`.

Notes
- This repository is intended for learning and experimentation — examples may be small, focused, and educational rather than production-ready.
- If you want, open issues or add short descriptions to each example file explaining the exact learning goal.

License & Author
- MIT-style permissive use (add a license file if desired).
- Author: Kshitij khatri
