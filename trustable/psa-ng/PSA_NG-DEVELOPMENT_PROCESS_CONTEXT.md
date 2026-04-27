---
normative: false
---

**Guidance**

Automated code quality enforcement ensures that every change meets baseline standards before it reaches the main branch. The CI check workflow runs formatting verification (rustfmt), linting (clippy with warnings denied), compilation checks, full test suite execution, documentation generation, and dependency auditing (cargo-deny) on every PR and push to main.

**Evidence**

Evidence for this statement could include:

* `github` reference to `.github/workflows/check.yaml` showing the complete check pipeline
* `github` reference to `.github/workflows/nightly.yaml` showing extended checks (MSRV, locked audit)
* `openfasttrace` trace linking `req~rust-best-practices` to its implementation
* CI run logs showing PR check enforcement

**Confidence scoring**

Score of 0.6 reflects that CI enforcement is verifiable but branch protection rules (requiring checks to pass before merge) are a repository setting configured outside the codebase itself.
