<p align="center">
    <img src="https://raw.githubusercontent.com/orhun/binsider/main/website/src/assets/binsider-logo-dark.png#gh-dark-mode-only" width="300"></a>
    <img src="https://raw.githubusercontent.com/orhun/binsider/main/website/src/assets/binsider-logo-light.png#gh-light-mode-only" width="300"></a>
    <br>
    <img src="https://raw.githubusercontent.com/orhun/binsider/main/website/src/assets/binsider-text-dark.png#gh-dark-mode-only" width="170"></a>
    <img src="https://raw.githubusercontent.com/orhun/binsider/main/website/src/assets/binsider-text-light.png#gh-light-mode-only" width="170"></a>
    <br>
    <br>
    <em>"Swiss army knife for reverse engineers."</em>
    <br>
    <br>
    <a href="https://github.com/orhun/git-cliff/releases">
        <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&labelColor=1d1d1d&color=f8be75&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/git-cliff/">
        <img src="https://img.shields.io/crates/v/git-cliff?style=flat&labelColor=1d1d1d&color=f8be75&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/orhun/git-cliff">
        <img src="https://img.shields.io/codecov/c/gh/orhun/git-cliff?style=flat&labelColor=1d1d1d&color=f8be75&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Integration%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/ci.yml?style=flat&labelColor=1d1d1d&color=white&logo=GitHub%20Actions&logoColor=white"></a>
    <a href="https://github.com/orhun/git-cliff/actions?query=workflow%3A%22Continuous+Deployment%22">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/cd.yml?style=flat&labelColor=1d1d1d&color=white&logo=GitHub%20Actions&logoColor=white&label=deploy"></a>
    <a href="https://hub.docker.com/r/orhunp/git-cliff">
        <img src="https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/docker.yml?style=flat&labelColor=1d1d1d&color=white&label=docker&logo=Docker&logoColor=white"></a>
    <a href="https://docs.rs/git-cliff-core/">
        <img src="https://img.shields.io/docsrs/git-cliff-core?style=flat&labelColor=1d1d1d&color=white&logo=Rust&logoColor=white"></a>
    <br>
</p>

<h4 align="center">
  <a href="https://binsider.dev/getting-started/">Documentation</a> |
  <a href="https://binsider.dev/">Website</a>
</h4>

😼🕵️‍♂️ **Binsider** can perform static and dynamic analysis, inspect strings, examine linked libraries, and perform hexdumps, all within a user-friendly terminal user interface!

## Quickstart

Install `binsider` with `cargo`:

```bash
cargo install binsider
```

> [!NOTE]  
> See the other [installation methods](https://binsider.dev/installation/crates-io/) 📦

After the installation, you are pretty much set! 💯

Just dive into the binaries by running `binsider`:

```bash
binsider <binary>
```

![Demo](website/src/content/assets/quickstart.gif)

## Features

The detailed documentation is available at <https://binsider.dev>.

### General Analysis

You can retrieve general binary file information, including file size, ownership, permissions, date, and linked shared libraries (similar to [`stat(1)`](https://www.man7.org/linux/man-pages/man1/stat.1.html) and [`ldd(1)`](https://www.man7.org/linux/man-pages/man1/ldd.1.html)).

[![General analysis](website/src/assets/demo/binsider-general-analysis.gif)](https://binsider.dev/usage/general-analysis)

<p align="center">

<https://binsider.dev/usage/general-analysis>

</p>

### Static Analysis

You can analyze the ELF layout (such as sections, segments, symbols, and relocations) and navigate through them to get an in-depth understanding of the binary.

[![Static analysis](website/src/assets/demo/binsider-static-analysis.gif)](https://binsider.dev/usage/static-analysis)

<p align="center">

<https://binsider.dev/usage/static-analysis>

</p>

### Dynamic Analysis

It is possible to execute the binary and trace the system calls, signals, and the program's execution flow similar to [`strace(1)`](https://man7.org/linux/man-pages/man1/strace.1.html) and [`ltrace(1)`](https://man7.org/linux/man-pages/man1/ltrace.1.html).

[![Dynamic analysis](website/src/assets/demo/binsider-dynamic-analysis.gif)](https://binsider.dev/usage/dynamic-analysis)

<p align="center">

<https://binsider.dev/usage/dynamic-analysis>

</p>

### String Extraction

Similar to the [`strings(1)`](https://linux.die.net/man/1/strings) command, `binsider` is able to extract strings from the binary file with the purpose of discovering interesting strings such as URLs, passwords, and other sensitive information.

[![String extraction](website/src/assets/demo/binsider-strings.gif)](https://binsider.dev/usage/strings)

<p align="center">

<https://binsider.dev/usage/strings>

</p>

### Hexdump

`binsider` provides a rich dashboard along with a hexdump view to analyze the binary content in a structured manner.

[![Hexdump](website/src/assets/demo/binsider-hexdump.gif)](https://binsider.dev/usage/hexdump)

<p align="center">

<https://binsider.dev/usage/hexdump>

</p>

## Acknowledgements

Shoutout to [Harun Ocaksız](https://instagram.com/harunocaksiz) for sticking with me during our military service in the summer of 2024 and creating the awesome **binsider** logo! (o7)

## Contributing

See the [contribution guidelines](CONTRIBUTING.md).

## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
