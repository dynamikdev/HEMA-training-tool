The following standards represent the professional consensus for high-performance, maintainable Rust codebases.

## 1. Automated Style Enforcement
The foundation of any modern Rust project is zero-tolerance for manual formatting debates.

* **`rustfmt`**: All code must be formatted using the default `rustfmt` configuration. Do not customize it unless there is a project-critical reason. Run it on every commit or via a CI check: `cargo fmt --all -- --check`.
* **`clippy`**: Use the Lints provided by Clippy to catch common mistakes and non-idiomatic patterns.
    * In the `Cargo.toml` or at the crate root, use: `#![deny(clippy::all, clippy::pedantic)]`.
    * For mission-critical code, consider `#![deny(missing_docs)]`.


## 2. Naming Conventions
Rust uses specific casing to distinguish between types and values.

| Item | Case | Example |
| :--- | :--- | :--- |
| **Crates / Modules** | `snake_case` | `data_processor` |
| **Types / Structs / Enums** | `PascalCase` | `UserAccount` |
| **Traits** | `PascalCase` | `StreamHandler` |
| **Functions / Methods** | `snake_case` | `fetch_record()` |
| **Variables** | `snake_case` | `retry_count` |
| **Constants / Statics** | `SCREAMING_SNAKE_CASE` | `MAX_TIMEOUT` |
| **Type Parameters** | `PascalCase` (usually single letter) | `T`, `U`, `Item` |


## 3. Project Structure & API Design
Modern Rust favors composition and clear visibility boundaries.

* **Module Hierarchy**: Use the 2018/2021/2024 edition style. Prefer `mod.rs`-less directory structures (e.g., `src/network.rs` and a folder `src/network/` for submodules).
* **Constructor Patterns**: Use `Default` where a "zeroed" state makes sense. Use a static `new()` method for primary initialization.
    * *Critique:* Avoid "Builder" patterns for simple structs; only use them when the number of optional parameters exceeds 4-5 to avoid "argument soup."
* **Visibility**: Default to `pub(crate)` or private. Only use `pub` for items intended to be part of the public API.


## 4. Error Handling
Rust has moved away from "stringly-typed" errors.

* **Libraries**: Use the `thiserror` crate to define custom, structured error enums.
* **Applications**: Use the `anyhow` crate for high-level error handling where you don't need to programmatically distinguish between error types.
* **Avoid `unwrap()`**: Use `expect("Reason why this cannot fail")` if a failure truly represents a bug. In all other cases, propagate errors using the `?` operator.



## 5. Memory & Performance Idioms
* **Ownership**: Prefer passing references (`&T`) rather than moving ownership unless the function requires it.
* **Slices vs. Collections**: Always prefer `&[T]` over `&Vec<T>` and `&str` over `&String` in function arguments to maximize flexibility.
* **Zero-Copy**: Utilize `Cow<'a, str>` (Clone-on-Write) when a function might return either a borrowed string or a newly allocated one.
* **Concurrency**: Favor `Send` and `Sync` traits to ensure thread safety. Use `Arc<T>` only when shared ownership across threads is strictly necessary; otherwise, prefer scoped threads or message passing (`mpsc`).


## 6. Documentation
Documentation is treated as code in Rust.

* **Doc Comments**: Use `///` for items and `//!` for crate/module-level docs.
* **Examples**: Every public function should ideally have an `/// # Examples` section. These are run as tests during `cargo test`, ensuring the documentation never goes out of sync with the implementation.
* **Panics/Errors**: Explicitly document when a function might panic using the `/// # Panics` header.

