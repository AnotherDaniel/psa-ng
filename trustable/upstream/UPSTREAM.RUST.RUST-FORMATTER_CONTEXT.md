---
level: 3.6.1
normative: false
---

The Rust tooling ecosystem provides the `cargo fmt` tool for fully automated and unambiguous code formatting:

**What is cargo fmt:**

Rust's official code formatter that automatically formats code according to standard style guidelines. It's built on rustfmt and ensures consistent formatting across your entire codebase without manual intervention.

**Key Benefits:**

*Zero Configuration* - Works out of the box with sensible defaults. No bikeshedding about brace placement, indentation, or line length.

*Complete Consistency* - Every file follows identical formatting rules, regardless of who wrote it or when. Eliminates style variations between team members.

*Mental Load Reduction* - Developers stop thinking about formatting entirely. Write code however feels natural, then let fmt handle the presentation.

*Review Focus* - Code reviews become about logic and design rather than "move this brace" or "fix indentation" comments.

**Why Include in Merge Checks:**

*Enforced Standards* - Prevents inconsistently formatted code from entering the main branch. No exceptions, no human judgment calls.

*Diff Cleanliness* - Formatted code produces cleaner git diffs focused on actual changes rather than formatting noise.

*Tool Integration* - IDEs and editors work better with consistently formatted code for features like folding, navigation, and refactoring.

*Onboarding Simplification* - New team members don't need to learn project-specific style rules.

**Implementation Strategy:**

Run `cargo fmt --check` in CI to verify formatting without modifying files. Most teams also set up pre-commit hooks or editor integration for automatic formatting during development.

**Practical Workflow:**

Developers run `cargo fmt` locally before committing, and CI catches any missed formatting. Some teams prefer automatic formatting in pre-push hooks to reduce friction.

**Customization:**

While rustfmt is opinionated, you can adjust settings in rustfmt.toml for project-specific needs, though the defaults work well for most teams.

The key advantage is removing formatting as a source of friction entirely.
