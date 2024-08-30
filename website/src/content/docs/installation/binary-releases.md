---
title: Binary releases
sidebar:
  order: 4
---

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/binsider/releases).

:::note

Release tarballs are signed with the following PGP key: [1D2D410A741137EBC544826F4A92FA17B6619297](https://keyserver.ubuntu.com/pks/lookup?search=0x4A92FA17B6619297&op=vindex)

:::

1. Download the latest binary from [releases](https://github.com/orhun/binsider/releases). You can pick between [glibc](https://en.wikipedia.org/wiki/Glibc) or [musl-libc](https://musl.libc.org/) compiled versions.

2. To download the binary compiled with `glibc`:

```bash
VERSION="0.1.0"
wget "https://github.com/orhun/binsider/releases/download/v${VERSION}/binsider-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
```

2. To download the binary compiled with `musl-libc`:

```bash
VERSION="0.1.0"
wget "https://github.com/orhun/binsider/releases/download/v${VERSION}/binsider-${VERSION}-x86_64-unknown-linux-musl.tar.gz"
```

3. Extract the files:

```bash
tar -xvzf binsider-*.tar.gz
```

4. Enter the folder and run the binary:

```bash
cd "binsider-${version}"
./binsider
```

5. Move binary to `/usr/local/bin/` (optional).
