---
normative: true

publish:
    group: "Security"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng web interface applies defence-in-depth hardening: all dynamic content is HTML-escaped to prevent cross-site scripting, security headers (X-Content-Type-Options, X-Frame-Options, Referrer-Policy, Content-Security-Policy) are set on every response, request body size is limited to 64 KB, error responses do not expose internal paths or URLs, and dependency vulnerabilities are audited in CI.
