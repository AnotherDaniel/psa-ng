---
normative: false
---

**Guidance**

The PSA Connected Car v4 API requires a specific workflow for remote commands: a callback must be registered first via `POST /user/callbacks`, and its returned ID must be used in subsequent remote action requests at `POST /user/vehicles/{id}/callbacks/{cbid}/remotes`. Remote payloads must use the documented JSON schema with typed action fields (`door`, `horn`, `charging`, `lights`, `wakeUp`, `preconditioning`). API errors follow a structured format with enhanced HTTP error codes, UUIDs for support, and timestamps.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~callback-registration`, `req~remote-command-schema`, and `req~api-error-parsing` to their implementations and unit tests
* `github` reference to the PSA API client module showing callback management and remote command construction
* Unit test results verifying correct endpoint paths, payload schemas, and error parsing

**Confidence scoring**

Score of 0.8 reflects that protocol conformance is directly testable via mock HTTP responses and payload inspection, giving high confidence when tests pass.
