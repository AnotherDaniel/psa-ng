---
normative: true

publish:
    group: "PSA API"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng API client handles API rate limiting by parsing X-RateLimit and Retry-After response headers and delaying requests on HTTP 429, supports token-based pagination for collection endpoints to retrieve complete result sets, and requests appropriate OAuth2 scopes during authorization.
