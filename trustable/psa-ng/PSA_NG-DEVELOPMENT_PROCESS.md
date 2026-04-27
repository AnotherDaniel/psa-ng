---
normative: true

publish:
    group: "Process"
#EVIDENCE_REF#
score: 
    Developer: 0.6
---

The psa-ng project enforces code quality through automated CI checks (formatting via rustfmt, linting via clippy with deny-warnings, compilation checks, and cargo-deny dependency auditing) on every pull request and push to main, blocking merges that fail any check.
