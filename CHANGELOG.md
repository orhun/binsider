<img src="https://raw.githubusercontent.com/orhun/binsider/main/website/src/assets/binsider-logo-dark.png" width="200"></a>

[**binsider**](https://binsider.dev): Analyze ELF binaries like a boss 😼🕵️‍♂️

## 0.2.1 - 2024-10-10

### 🚀 Features

- *(tui)* Use stdout for rendering by @orhun in [#79](https://github.com/orhun/binsider/pull/79)
- *(ui)* Support shift+tab for going to the previous tab by @XXMA16 in [#70](https://github.com/orhun/binsider/pull/70)
- *(cli)* Add `--tab` argument by @josueBarretogit in [#60](https://github.com/orhun/binsider/pull/60)
- *(general)* Display the number of shared libraries by @sumit0190 in [#58](https://github.com/orhun/binsider/pull/58)

### 🐛 Bug Fixes

- *(tui)* [**breaking**] Query the terminal background once by @orhun in [#62](https://github.com/orhun/binsider/pull/62)
- *(flake)* Add missing meta section to flake by @ch4og in [#74](https://github.com/orhun/binsider/pull/74)
- *(cd)* Enable cross compilationby @orhun

### ⚡ Performance

- *(flake)* Speed up rebuild by using naersk by @ch4og in [#76](https://github.com/orhun/binsider/pull/76)

### ⚙️ Miscellaneous Tasks

- *(deny)* Update ignored advisoriesby @orhun
- *(ci)* Add nix flake build by @ch4og in [#75](https://github.com/orhun/binsider/pull/75)

## New Contributors
* @XXMA16 made their first contribution in [#70](https://github.com/orhun/binsider/pull/70)
* @josueBarretogit made their first contribution in [#60](https://github.com/orhun/binsider/pull/60)
* @sumit0190 made their first contribution in [#58](https://github.com/orhun/binsider/pull/58)

**Full Changelog**: https://github.com/orhun/binsider/compare/v0.2.0...0.2.1

## 0.2.0 - 2024-09-30

✨ See the blog post about this release: <https://binsider.dev/blog/v020/>

### 🚀 Features

- *(ui)* Add loading/splash screen  by @orhun in [#55](https://github.com/orhun/binsider/pull/55)
- *(lib)* Add example/documentation about using as a library  by @orhun in [#52](https://github.com/orhun/binsider/pull/52)
- *(dynamic)* Support running binaries with CLI arguments  by @orhun in [#49](https://github.com/orhun/binsider/pull/49)
- *(static)* Reorder symbol table for better readability  by @orhun in [#42](https://github.com/orhun/binsider/pull/42)
- *(dynamic)* Make dynamic analysis optional for better platform support  by @orhun in [#31](https://github.com/orhun/binsider/pull/31)
- *(tui)* Improve the white theme support  by @orhun in [#23](https://github.com/orhun/binsider/pull/23)
- *(nix)* Add a simple flake.nix  by @jla2000 in [#14](https://github.com/orhun/binsider/pull/14)

### 🐛 Bug Fixes

- *(ui)* Avoid crashing when logo does not fit the terminal by @orhun
- *(test)* Update file info arguments by @orhun
- *(dynamic)* Fix locating the binary  by @orhun in [#48](https://github.com/orhun/binsider/pull/48)
- *(dynamic)* Sort the shared library list  by @orhun in [#37](https://github.com/orhun/binsider/pull/37)
- *(strings)* Replace unicode whitespace for correct rendering  by @orhun in [#28](https://github.com/orhun/binsider/pull/28)
- *(file)* Do not panic if creation time is not supported  by @orhun in [#25](https://github.com/orhun/binsider/pull/25)
- *(tui)* Stop the event handler on quit  by @orhun in [#24](https://github.com/orhun/binsider/pull/24)
- *(flake)* Fix test failure on Nix  by @ch4og in [#30](https://github.com/orhun/binsider/pull/30)
- *(docker)* Fix inconsistent keyword casing by @orhun
- *(ci)* Only run library unit tests in CI by @orhun
- *(test)* Ensure that binary is built before the test runs  by @samueltardieu in [#11](https://github.com/orhun/binsider/pull/11)
- *(website)* Handle GitHub release version correctly by @orhun

### 📚 Documentation

- *(blog)* Add blog post for 0.2.0 release  by @orhun in [#53](https://github.com/orhun/binsider/pull/53)

### ⚙️ Miscellaneous Tasks

- *(website)* Add discord link by @orhun
- *(changelog)* Update git-cliff config by @orhun
- *(website)* Do not deploy website for pull requests by @orhun

## New Contributors

* @ch4og made their first contribution in [#30](https://github.com/orhun/binsider/pull/30)
* @samueltardieu made their first contribution in [#11](https://github.com/orhun/binsider/pull/11)
* @jla2000 made their first contribution in [#14](https://github.com/orhun/binsider/pull/14)

**Full Changelog**: https://github.com/orhun/binsider/compare/v0.1.0...v0.2.0

## 0.1.0 - 2024-09-11

Initial release 🚀
