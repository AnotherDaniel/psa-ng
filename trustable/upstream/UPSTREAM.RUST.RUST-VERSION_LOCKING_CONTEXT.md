---
level: 3.3.1
normative: false
---

A Rust project can explicitly determine how it deals with its dependencies depending on whether `Cargo.lock` is managed in version control or not:

Tracking `Cargo.lock` in version control ensures reproducible builds. This guarantees that everyone working on the project and your deployment environments use exactly the same dependency versions. Without it, `cargo build` might pull newer versions that could introduce breaking changes or subtle bugs between environments.

This is true also for libraries - although they are meant to be flexible with their dependency versions (within the constraints specified in `Cargo.toml`), a Rust application build will ignore the `Cargo.lock`files of its dependencies/crates anyways and perform its own dependency version resolution, so for users of a library it does not make a difference whether `Cargo.lock` is tracked by the library or not.

Build reproducibility and predictable test environments are a priority for trustable software, therefore it is always recommended to track `Cargo.lock`in version control.
