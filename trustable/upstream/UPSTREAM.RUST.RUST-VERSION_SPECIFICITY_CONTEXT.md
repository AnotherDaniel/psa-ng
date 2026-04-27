---
level: 3.2.1
normative: false
---

A Rust project can choose via its `Cargo.toml` how specific is is when referencing the version of dependencies to use:

**For Applications/Binaries:**

Use relatively specific version constraints, but avoid exact pinning unless absolutely necessary. Common approaches:

- `serde = "1.0.195"` (allows patch updates: 1.0.195 to 1.0.x)
- `tokio = "1.35"` (allows minor updates: 1.35.x to 1.x.x)
- Avoid `serde = "=1.0.195"` (exact pinning) unless you've found a specific breaking change

**For Libraries/Crates:**

Be as permissive as reasonable to maximize compatibility with downstream users:

- `serde = "1.0"` (allows all 1.x versions)
- `tokio = "1.0"` (accepts any 1.x version)
- Only add upper bounds when you know of actual incompatibilities

**Key Considerations:**

*Dependency Hell Prevention* - Overly restrictive library constraints create impossible-to-resolve dependency graphs when multiple libraries depend on the same crate with conflicting requirements.

*Security vs Stability* - Looser constraints mean you get security patches automatically via `cargo update`, but tighter constraints give more predictable behavior. The `Cargo.lock` file already provides the stability for applications.

*Breaking Change Risk* - Rust's semantic versioning means minor/patch updates shouldn't break a project, but reality is messier. Popular, well-maintained crates (like serde, tokio) are generally safe to trust with looser constraints.

*MSRV Compatibility* - If you specify a Minimum Supported Rust Version, ensure your dependency constraints align with versions that support that MSRV.

**General Strategy:**

Start permissive, then tighten only when you encounter actual problems. Let Cargo.lock handle exact reproducibility while keeping Cargo.toml constraints as flexible as your compatibility requirements allow.
