---
normative: false
---

**Guidance**

Web hardening addresses multiple OWASP Top 10 concerns for an application that controls physical vehicles. XSS prevention ensures that API-sourced data (VIN, brand, charging status) cannot inject scripts. Security headers mitigate clickjacking, MIME sniffing, and information leakage. Body size limits prevent denial-of-service via large payloads. Error sanitization prevents internal implementation details from being exposed to clients. Dependency auditing catches known CVEs before release.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` traces linking `req~html-output-escaping`, `req~security-headers`, `req~sanitized-errors`, `req~request-body-limit`, and `req~dependency-audit` to their implementations
* `github` references to the escape_html function in templates.rs and security_headers_middleware in routes.rs
* Unit tests verifying security headers are present and error responses are sanitized
* CI step running `cargo audit` to detect dependency vulnerabilities

**Confidence scoring**

Score of 0.7 reflects that while individual measures are straightforward to verify, comprehensive defence-in-depth requires ongoing review as the application evolves.
