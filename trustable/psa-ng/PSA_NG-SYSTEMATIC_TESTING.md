---
normative: true

publish:
    group: "Testing"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng project validates behaviour through systematic, scheduled test execution: all unit and integration tests run on every pull request and push to main, a nightly CI workflow repeats the full test suite on a schedule to detect flaky tests and environment drift, and test coverage reports are generated on every release to confirm that exercised code paths remain stable over time.

