---
normative: false
---

**Guidance**

The web interface serves two purposes: a REST API for programmatic access and an interactive dashboard for human users. The REST API must return proper JSON for all vehicle operations, while the dashboard pages must render usably across common screen sizes (mobile 320px through desktop 1920px) using responsive CSS.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking web server endpoint requirements (`req~vehicle-status-endpoint`, `req~charge-control-endpoint`, etc.) and dashboard requirements (`req~dashboard-overview`, `req~charge-management-page`, etc.) to their implementations
* `github` reference to the web server and template source files
* Unit test results for REST endpoint response correctness

**Confidence scoring**

Score of 0.7 reflects that REST endpoint correctness is highly testable, while dashboard usability and styling quality are more subjective.
