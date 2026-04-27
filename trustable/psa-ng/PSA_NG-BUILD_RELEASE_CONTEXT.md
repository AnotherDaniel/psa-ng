---
normative: false
---

**Guidance**

A reproducible build and release process is essential for construction confidence. The project must compile from source using the standard Rust toolchain, with all quality gates (tests, linting) passing before artifacts are published.

**Evidence**

Evidence for this statement could include:

* `github` reference to `.github/workflows/release.yaml` showing the CI pipeline with test and clippy steps
* `github` reference to `Cargo.toml` workspace configuration
* `openfasttrace` trace linking `req~rust-best-practices` to its implementation

**Confidence scoring**

Score of 0.7 reflects that the CI pipeline is directly inspectable and automated, though full reproducibility verification (bit-for-bit builds) is not yet in scope.
