---
normative: true

publish:
    group: "Provenance"
#EVIDENCE_REF#
score: 
    Developer: 0.6
---

All third-party dependencies of psa-ng are sourced exclusively from crates.io (enforced by cargo-deny source checks), pinned to exact versions via Cargo.lock, and scanned for known CVEs by cargo-deny on every CI run with no unacknowledged critical or high-severity advisories at the time of release.

