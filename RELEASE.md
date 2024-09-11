# Creating a Release

[GitHub](https://github.com/orhun/binsider/releases) and [crates.io](https://crates.io/crates/binsider/) releases are automated via [GitHub actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Bump the version in [Cargo.toml](Cargo.toml) according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
2. Update [Cargo.lock](Cargo.lock) by building the project: `cargo build`
3. Update [CHANGELOG.md](CHANGELOG.md) by running [`git-cliff`](https://git-cliff.org).
4. Commit your changes.
5. Create a new tag: `git tag -s -a v[X.Y.Z]`
6. Push the tag: `git push --tags`
7. Announce the release! 🥳
