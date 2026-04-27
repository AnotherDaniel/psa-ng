---
normative: true

publish:
    group: "PSA API"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng application maps PSA API errors, network failures, and invalid inputs to specific HTTP status codes (502 for upstream failures, 401 for auth errors, 500 for internal errors) and returns error messages that exclude file paths, URLs, and token values.

