---
normative: true

publish:
    group: "PSA API"
#EVIDENCE_REF#
score: 
    Developer: 0.8
---

The psa-ng project retrieves vehicle status (battery level, charging state, odometer, position) and executes remote commands (charging control, preconditioning, door locks, lights, horn) via the PSA Connected Car v4 API, with each operation covered by unit tests that verify HTTP request construction and JSON response parsing against mock responses.
