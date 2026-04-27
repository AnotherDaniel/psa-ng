---
normative: false
---

**Guidance**

OAuth2 authentication is the foundation of all PSA API interactions. The implementation must handle the full authorization code flow, including constructing proper authorization URLs, exchanging codes for tokens, and seamlessly refreshing expired tokens. Token persistence ensures users are not forced to re-authenticate on every application restart.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~oauth2-authentication`, `req~token-refresh`, and `req~credential-persistence` to their implementations and unit tests
* `github` reference to the OAuth2 client module source code
* Unit test results demonstrating correct token lifecycle management

**Confidence scoring**

Score of 0.8 reflects that OAuth2 is well-understood and testable, though full integration testing against the live PSA API depends on external service availability.
