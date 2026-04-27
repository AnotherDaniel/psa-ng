---
normative: false
---

**Guidance**

Credential security is critical because the application handles OAuth2 tokens that grant remote control of physical vehicles. Mishandled credentials could allow unauthorized vehicle access. The application must ensure credentials are stored securely, not leaked through logs, and only transmitted over encrypted channels.

**Evidence**

Evidence for this statement could include:

* `openfasttrace` trace linking `req~credential-persistence` to its implementation
* `github` reference to credential storage implementation showing Unix 0o600 file permissions on token files
* Code review confirming no credential values appear in log output
* Code review confirming all PSA API HTTP calls use HTTPS URLs

**Confidence scoring**

Score of 0.7 reflects that secure local storage and HTTPS enforcement are straightforward to verify, but comprehensive log auditing requires ongoing vigilance.
