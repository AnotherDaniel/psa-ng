---
normative: false
---

**Guidance**

The TSF framework requires that each released iteration of software includes not just code but also build instructions, tests, results, and attestations. The psa-ng release workflow produces all of these as artifacts attached to each tagged release: source code (via git tag), build instructions (Cargo.toml, Dockerfile, docker-compose.yaml), test execution (CI check gate), OFT tracing report (aspec + HTML), TSF evidence package (tsffer snippets), and the published trust report (tsflink output).

**Evidence**

Evidence for this statement could include:

* `github` reference to `.github/workflows/release.yaml` showing the full pipeline from check gate through artifact publication
* `openfasttrace` trace confirming all requirements pass before release
* `github` reference to `Dockerfile` and `docker-compose.yaml` as build/deployment instructions
* `github` reference to `Cargo.toml` as the authoritative build configuration

**Confidence scoring**

Score of 0.7 reflects that the release pipeline is fully automated and includes all major artifacts, though formal attestation signing (e.g. Sigstore/cosign) is not yet implemented.
