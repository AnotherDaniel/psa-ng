---
normative: false
---

**Guidance**

Change management ensures that all modifications to the codebase are tracked, reviewed, and verified. For psa-ng this means version-controlled commits, dependency updates via Cargo, and regression testing through the CI pipeline before any release.

**Evidence**

Evidence for this statement could include:

* `github` reference to the repository commit history showing version-controlled changes
* `github` reference to `.github/workflows/release.yaml` showing test execution before release
* `openfasttrace` trace linking implemented requirements to verify no regressions

**Confidence scoring**

Score of 0.6 reflects that git history and CI are auditable, but formal change review processes (pull request reviews, approval gates) are not yet enforced.
