---
normative: false
---

**Guidance**

Unit testing provides confidence that individual components behave correctly in isolation. For psa-ng, this includes testing API request construction, response parsing, web endpoint routing, and configuration loading. Tests should use mock HTTP responses rather than hitting the live PSA API.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking all `req~` items with `Needs: utest` to their corresponding test implementations
* `github` reference to test source files
* CI pipeline test execution results showing all tests pass

**Confidence scoring**

Score of 0.8 reflects that unit test coverage is directly measurable and automated CI execution ensures tests are consistently run.
