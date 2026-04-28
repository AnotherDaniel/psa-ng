---
normative: true

publish:
    group: "PSA API"
#EVIDENCE_REF#
score: 
    Developer: 0.8
---

The psa-ng API client conforms to the PSA Connected Car v4 API protocol by registering callbacks before sending remote commands, using the documented endpoint paths and JSON payload schemas for all remote operations, sending the correct Content-Type header, and parsing structured API error responses with code, uuid, message, and timestamp fields.
