---
title: Build from source
sidebar:
  order: 5
---

## Prerequisites

- [Cargo](https://www.rust-lang.org/)
  - The minimum supported Rust version (MSRV) is `1.74.1`.

## Instructions

1. Clone the repository.

```bash
git clone https://github.com/orhun/binsider
cd binsider/
```

2. Build.

```bash
CARGO_TARGET_DIR=target cargo build --release
```

Binary will be located at `target/release/binsider`.
