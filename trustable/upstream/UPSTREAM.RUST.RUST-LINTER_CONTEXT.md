---
level: 3.5.1
normative: false
---

The Rust tooling ecosystem provides the `cargo clippy` tool as a high-powered linter and coding style checker:

**What is Clippy:**

Clippy is Rust's official linting tool that catches common mistakes, suggests idiomatic code patterns, and enforces best practices. It goes beyond what the compiler checks to improve code quality and maintainability.

**Key Benefits:**

*Code Quality* - Catches subtle bugs the compiler misses, like incorrect use of floating-point comparisons, potential integer overflows, or inefficient string operations.

*Idiom Enforcement* - Promotes Rust best practices like using `?` operator instead of manual error handling, preferring iterators over manual loops, and following naming conventions.

*Performance Hints* - Identifies inefficient patterns like unnecessary clones, suboptimal data structure usage, or missing compiler optimizations.

*Readability* - Suggests cleaner code patterns that make intentions clearer to other developers.

**Why Include in Merge Checks:**

*Consistency* - Ensures all team members follow the same coding standards without requiring extensive code review guidelines.

*Knowledge Transfer* - Junior developers learn Rust idioms automatically through clippy suggestions rather than trial and error.

*Bug Prevention* - Many clippy lints catch real bugs before they reach production. Better to fix them at merge time than in production.

*Reduced Review Burden* - Reviewers can focus on logic and design rather than style nitpicks and common pitfalls.

**Implementation Strategy:**

Run `cargo clippy -- -D warnings` in CI to treat all clippy warnings as errors. You can customize severity levels or disable specific lints in clippy.toml if needed.

**Practical Consideration:**

Clippy can be noisy initially on existing codebases. Consider starting with `clippy::correctness` and `clippy::suspicious` categories, then gradually adding style lints as the team adapts.

The investment in CI setup pays dividends in code quality and team velocity over time.
