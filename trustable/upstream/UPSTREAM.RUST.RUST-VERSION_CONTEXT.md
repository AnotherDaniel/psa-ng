---
level: 3.1.1
normative: false
---

Rust provides the concept of MSRV for managing the scope of supported Rust versions:

**What is MSRV:**

Minimum Supported Rust Version - the oldest Rust compiler version your crate promises to work with. It's a compatibility guarantee a crate make to users who can't or won't upgrade to the latest Rust.

**Why It Matters:**

- Corporate environments often lag behind latest Rust releases
- Embedded or specialized toolchains may use older Rust versions
- Your users need predictability about what Rust version they need

**How to Manage MSRV:**

*Declaration* - Specify it in Cargo.toml:
```toml
[package]
rust-version = "1.70.0"
```

*Testing* - Run CI against your MSRV, not just latest Rust. Use `cargo +1.70.0 check` to verify compatibility.

*Dependency Constraints* - This is the tricky part. Your dependencies might require newer Rust versions than your MSRV. You need to either:

- Constrain dependency versions to ones that support your MSRV
- Bump your MSRV to match your dependencies' requirements

**Common Challenges:**

- Dependencies silently bump their MSRV in minor releases
- Transitive dependencies can break your MSRV without you realizing
- Balancing new language features against broad compatibility

**Practical Approach:**

Choose an MSRV based on your actual user needs, not arbitrary conservatism. Many projects target 6-12 months behind current stable. Popular crates often follow Rust's N-2 policy (supporting 2-3 versions back).

**Tools:**

`cargo-msrv` can help find the minimum version that actually works, and `cargo-semver-checks` can detect MSRV bumps in your releases.
