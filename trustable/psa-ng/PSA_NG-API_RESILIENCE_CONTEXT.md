---
normative: false
---

**Guidance**

The PSA API enforces rate limits (daily sliding window + burst per-second) and returns 429 with Retry-After headers when exceeded. Collection endpoints use token-based pagination with pageToken/pageSize parameters. The OAuth2 authorization must request appropriate scopes (data:telemetry, data:position, remote:door:write, etc.) for the operations the client intends to use.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~rate-limit-handling`, `req~api-pagination`, and `req~oauth2-scope-management` to their implementations and unit tests
* `github` reference to rate-limit retry logic and pagination iteration code
* Unit test results verifying 429 handling with backoff, multi-page retrieval, and scope request construction

**Confidence scoring**

Score of 0.7 reflects that rate limiting and pagination are testable via mock responses, though real-world behaviour depends on external API timing that cannot be fully replicated in unit tests.
