---
normative: false
---

**Guidance**

Dependency provenance ensures that all third-party code entering the project is from known, trusted sources. For a Rust project, this means dependencies are sourced from crates.io, locked to specific versions via Cargo.lock, and chosen for maturity and active maintenance.

**Evidence**

Evidence for this statement could include:

* `github` reference to `Cargo.lock` showing pinned dependency versions
* `github` reference to `deny.toml` showing source restrictions and advisory checks
* `openfasttrace` trace linking `req~stable-dependencies` to its implementation
* CI job output from `cargo-deny check` showing no unacknowledged advisories

**Confidence scoring**

Score of 0.6 reflects that dependency pinning is verifiable, but comprehensive supply chain attestation (SBOM, signature verification) is not yet implemented.
