---
normative: true

publish:
    group: "PSA API"
#EVIDENCE_REF#
score: 
    Developer: 0.8
---

The psa-ng project implements the OAuth2 authorization code flow for the PSA Connected Car v4 API: constructing brand-specific authorization URLs, exchanging authorization codes for access and refresh tokens, automatically refreshing expired access tokens before API calls, and persisting tokens to disk so that re-authentication is not required across restarts.
