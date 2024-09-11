# Contribution Guidelines

First off, thank you for considering contributing to `binsider`!

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting Issues

Before reporting an issue on the
[issue tracker](https://github.com/orhun/binsider/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull Requests

Try to do one pull request per change.

## Commit Message Format

This project adheres to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).
A specification for adding human and machine readable meaning to commit messages.

### Commit Message Header

```
<type>(<scope>): <short summary>
  │       │             │
  │       │             └─ Summary in present tense. Not capitalized. No period at the end.
  │       │
  │       └─ Commit Scope
  │
  └─ Commit Type: feat|fix|build|ci|docs|perf|refactor|test|chore
```

#### Type

| feat     | Features                 | A new feature                                                                                          |
| -------- | ------------------------ | ------------------------------------------------------------------------------------------------------ |
| fix      | Bug Fixes                | A bug fix                                                                                              |
| docs     | Documentation            | Documentation only changes                                                                             |
| style    | Styles                   | Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc) |
| refactor | Code Refactoring         | A code change that neither fixes a bug nor adds a feature                                              |
| perf     | Performance Improvements | A code change that improves performance                                                                |
| test     | Tests                    | Adding missing tests or correcting existing tests                                                      |
| build    | Builds                   | Changes that affect the build system or external dependencies (example scopes: main, serde)            |
| ci       | Continuous Integrations  | Changes to our CI configuration files and scripts (example scopes: Github Actions)                     |
| chore    | Chores                   | Other changes that don't modify src or test files                                                      |
| revert   | Reverts                  | Reverts a previous commit                                                                              |

## Developing

### Setup

This is no different than other Rust projects.

```bash
git clone https://github.com/orhun/binsider
cd binsider
cargo test
```

### Useful Commands

Run clippy:

```bash
cargo clippy --all-targets --all-features --workspace
```

Run all tests:

```bash
cargo test --all-features --workspace
```

Check to see if there are code formatting issues:

```bash
cargo fmt --all -- --check
```

Format the code in the project:

```bash
cargo fmt --all
```

## License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE).
