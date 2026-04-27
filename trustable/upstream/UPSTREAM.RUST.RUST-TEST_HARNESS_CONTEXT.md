---
level: 3.7.1
normative: false
---

The Rust core tool set includes the test framework plus associated tooling:

**Integrated Test Harness:**

Rust's test framework is built directly into the language and toolchain via `cargo test`. Tests are written as regular Rust functions marked with `#[test]`, and the compiler automatically generates a test runner. No external dependencies, configuration files, or separate test frameworks required.

For an example on how to integrate external testing methodologies with the core Rust test framework, look at how the behavior-driven [Gherkin](https://cucumber.io/docs/gherkin/) test framework can be integrated with the Rust test harness using [cucumber-rs](https://cucumber-rs.github.io/cucumber/main/).

**Core Integration Benefits:**

*Zero Setup Friction* - Every Rust project gets testing capability immediately. No decisions about which test framework to choose, how to configure it, or how to integrate it with the build system.

*Uniform Experience* - Whether you're working on a small library or large application, testing works identically. Documentation, tooling, and knowledge transfer seamlessly across projects.

*Built-in Conventions* - Standard patterns like unit tests in `src/`, integration tests in `tests/`, doc tests in documentation comments, and benchmarks in `benches/` are universally understood.

*Toolchain Consistency* - Test execution, filtering, parallel running, and output formatting are standardized. `cargo test --lib`, `cargo test integration_test`, and similar commands work everywhere.

**Documentation Testing:**

A unique benefit is executable documentation via doc tests. Code examples in /// comments are automatically tested, ensuring documentation stays accurate and examples actually compile and run.

**Performance and Reliability:**

The integrated runner handles parallel test execution, proper isolation, and efficient compilation. Since it's part of the core toolchain, it's highly optimized and battle-tested across the entire Rust ecosystem.

**Developer Workflow:**

Testing becomes a natural part of development rather than a separate concern. `cargo test` is as fundamental as `cargo build` or `cargo run`. IDE integration, CI setup, and developer habits all align around this single command.

**Ecosystem Alignment:**

Every crate on crates.io uses the same test infrastructure, making it trivial to contribute to open source projects or understand how any Rust codebase approaches testing.

This integration eliminates the "test framework fatigue" common in other ecosystems while ensuring testing is never an afterthought.
