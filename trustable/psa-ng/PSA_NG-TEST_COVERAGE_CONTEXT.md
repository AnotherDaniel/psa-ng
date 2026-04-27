---
normative: false
---

**Guidance**

Test coverage reporting provides quantitative evidence that the test suite exercises the codebase. While coverage alone does not prove correctness, it identifies untested code paths and tracks testing progress over time. The coverage report is generated using cargo-llvm-cov, which instruments the compiled binary for accurate line and region coverage.

**Evidence**

Evidence for this statement could include:

* `github` reference to release and nightly CI workflow files showing the coverage job configuration
* `download_url` reference to the published HTML coverage report artifact
* CI job output confirming successful cargo-llvm-cov execution

**Confidence scoring**

Score of 0.7 reflects that coverage report generation is fully automated and verifiable, though the coverage percentage itself is not yet gated (no minimum threshold enforced).
