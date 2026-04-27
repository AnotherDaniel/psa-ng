---
normative: false
---

**Guidance**

API access control prevents unauthorized remote control of connected vehicles. When an API token is configured, all REST endpoints require a valid Bearer token in the Authorization header. Browser-facing dashboard pages are exempt since they do not expose control APIs directly.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~api-bearer-auth` to its implementation
* `github` reference to the authentication middleware in routes.rs
* Unit tests verifying 401 responses for missing/invalid tokens and success with valid tokens

**Confidence scoring**

Score of 0.8 reflects that bearer token middleware is straightforward to verify and test, with the optional-when-unconfigured behaviour being a pragmatic trade-off documented in configuration.
