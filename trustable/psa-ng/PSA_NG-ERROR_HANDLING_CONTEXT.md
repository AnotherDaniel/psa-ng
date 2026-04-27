---
normative: false
---

**Guidance**

Robust error handling is critical for a vehicle control application. When the PSA API is unreachable, returns unexpected responses, or the user provides invalid input, the application must degrade gracefully — returning meaningful HTTP error codes and messages rather than panicking or leaking internal state.

**Evidence**

Evidence for this statement could include:

* `github` reference to `psa-api/src/error.rs` showing the structured error type hierarchy
* `github` reference to `psa-web/src/routes.rs` showing error-to-HTTP-status mapping in each handler
* `openfasttrace` traces for endpoint requirements that include error path testing
* Unit test results demonstrating graceful error handling for API failures

**Confidence scoring**

Score of 0.7 reflects that error paths are structurally present and testable, though exhaustive fault injection testing (network timeouts, malformed responses) is not yet comprehensive.
