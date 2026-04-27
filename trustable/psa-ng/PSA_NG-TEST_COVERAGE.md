---
normative: true

publish:
    group: "Testing"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng CI pipeline generates an HTML test coverage report on every release build and nightly run using cargo-llvm-cov, and publishes the report as a downloadable build artifact.
