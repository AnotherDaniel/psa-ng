---
normative: false
---

**Guidance**

Vehicle operations encompass both read operations (status, position, battery level) and write operations (charge control, preconditioning, door locks, lights, horn). Each operation must correctly construct HTTP requests to the PSA Connected Car v4 API and parse responses into typed Rust structures.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~vehicle-status`, `req~charge-control`, `req~preconditioning-control`, `req~door-lock-control`, and `req~lights-horn-control` to their implementations and unit tests
* `github` reference to the PSA API client module source code
* Unit test results verifying request construction and response deserialization

**Confidence scoring**

Score of 0.8 reflects that each operation is independently testable with mock HTTP responses, giving high confidence in correctness.
