---
title: Cargo
sidebar:
  order: 1
---

`binsider` can be installed from [crates.io](https://crates.io/crates/binsider) if you have [Rust](https://www.rust-lang.org/) installed:

```bash
cargo install binsider
```

If you want to install the latest git version:

```bash
cargo install --git https://github.com/orhun/binsider
```

:::note

The minimum supported Rust version (MSRV) is `1.74.1`.

:::

## Features

`binsider` supports the following feature flags which can be enabled or disabled during installation:

- `dynamic-analysis`: Enables the [dynamic analysis](/usage/dynamic-analysis) feature. (default: enabled)

e.g. To install `binsider` with the `dynamic-analysis` feature disabled:

```bash
cargo install binsider --no-default-features
```
