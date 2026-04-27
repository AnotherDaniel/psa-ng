---
normative: true

publish:
    group: "Security"
#EVIDENCE_REF#
score: 
    Developer: 0.8
---

The psa-ng project enforces optional bearer token authentication on all REST API endpoints, requiring a valid Authorization header when an API token is configured, and returning HTTP 401 for missing or invalid credentials.
