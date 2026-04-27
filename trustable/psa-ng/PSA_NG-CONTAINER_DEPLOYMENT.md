---
normative: true

publish:
    group: "Construction"
#EVIDENCE_REF#
score: 
    Developer: 0.7
---

The psa-ng project provides a multi-stage Dockerfile that builds the application from source using the stable Rust toolchain and a Docker Compose configuration for deployment, with a minimal runtime image, non-root execution, persistent data volume, and read-only configuration mount.