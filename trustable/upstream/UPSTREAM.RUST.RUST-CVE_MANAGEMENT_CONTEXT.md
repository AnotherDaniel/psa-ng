---
level: 3.4
normative: false
---

The Rust tooling ecosystem provides the `cargo deny` tool for managing allowed licenses and known vulnerabilities of dependencies:

**What is cargo-deny:**

A linting tool that helps enforce policies across your dependency graph. It's like a security and compliance gate that prevents problematic dependencies from entering your supply chain.

**CVE Management:**

The `advisories` section in deny.toml lets you automatically detect known security vulnerabilities:

```toml
[advisories]
database-path = "~/.cargo/advisory-db"
database-urls = ["https://github.com/rustsec/advisory-db"]
vulnerability = "deny"
unmaintained = "warn"
```

This pulls from RustSec's advisory database and fails your build if vulnerable dependencies are detected.

**Supply Chain Control:**

Beyond CVEs, cargo-deny helps with broader supply chain hygiene:

*License Compliance* - Reject dependencies with incompatible licenses:

```toml
[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0"]
deny = ["GPL-3.0"]
```

*Dependency Bloat* - Prevent duplicate dependencies or unwanted crates:

```toml
[bans]
multiple-versions = "deny"
deny = [
    { name = "openssl", reason = "Use rustls instead" }
]
```

**Workflow Integration:**

Run `cargo deny check` in CI to catch issues before they reach production. It's fast and integrates well with existing Rust toolchains.

**Practical Benefits:**

- Automated vulnerability scanning without external services
- Policy enforcement that scales across teams
- Early detection of supply chain drift
- Audit trail of security decisions in your deny.toml

**Limitations:**

Only catches known, disclosed vulnerabilities. Combine with other tools like `cargo audit` and dependency review processes for comprehensive security coverage.

The key advantage is making security policy explicit and automatically enforced rather than relying on manual reviews.
