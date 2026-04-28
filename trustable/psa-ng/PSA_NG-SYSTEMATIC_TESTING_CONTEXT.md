---
normative: false
---

**Guidance**

Systematic testing means running the full test suite not just on code changes but also on a schedule, to catch environment drift, flaky tests, and regressions introduced by dependency updates. Combined with coverage reporting, this validates that the exercised code paths remain stable and representative of real usage.

**Evidence**

Evidence for this statement could include:

* `github` reference to `.github/workflows/check.yaml` showing test execution on every PR and push
* `github` reference to `.github/workflows/nightly.yaml` showing scheduled nightly test runs
* `github` reference to `.github/workflows/release.yaml` showing coverage report generation via cargo-llvm-cov
* `openfasttrace` trace linking `req~rust-best-practices` to verify test pass requirement

**Confidence scoring**

Score of 0.7 reflects that scheduled and PR-triggered testing provides strong systematic validation, though stress testing (load, concurrency) is not yet part of the suite.
