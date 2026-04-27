---
normative: false
---

**Guidance**

Container-based deployment ensures reproducible, isolated execution of the application. A multi-stage build keeps the runtime image minimal (no compiler or build tools), while Docker Compose provides a declarative deployment configuration. Security best practices include running as a non-root user, mounting configuration read-only, and using named volumes for persistent data.

**Evidence**

Evidence for this statement could include:

* `github` reference to the `Dockerfile` showing the multi-stage build with stable Rust toolchain, minimal runtime image, and non-root `USER psa`
* `github` reference to `docker-compose.yaml` showing bind-mounted configuration, persistent named volume, and port mapping
* `openfasttrace` trace linking `req~container-image` and `req~container-deployment` to their implementation markers

**Confidence scoring**

Score of 0.7 reflects that the container build and deployment configuration are directly inspectable and follow established Docker best practices, though production hardening (read-only filesystem, resource limits, health checks) is not yet fully addressed.
