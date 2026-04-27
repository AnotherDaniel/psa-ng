---
normative: true

publish:
    group: "Security"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng project stores OAuth2 tokens in local files with Unix permission mode 0o600, never writes credential values to log output, and transmits credentials exclusively over HTTPS to the PSA identity provider and API endpoints.
