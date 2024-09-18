---
title: Binary releases
sidebar:
  order: 4
---

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/binsider/releases).

:::note

- Release tarballs for Linux/macOS are signed with the following PGP key: [0C9E792408F77819866E47FA85EF5848473D7F88](https://keyserver.ubuntu.com/pks/lookup?search=0x85EF5848473D7F88&op=vindex)
- If you are using Windows, you can simply download the zip file from the [releases page](https://github.com/orhun/binsider/releases).

:::

1. Download the binary from [releases](https://github.com/orhun/binsider/releases):

```bash
VERSION="0.1.0"
TARGET="x86_64-unknown-linux-gnu.tar.gz"
wget "https://github.com/orhun/binsider/releases/download/v${VERSION}/binsider-${VERSION}-${TARGET}.tar.gz"
```

2. Extract the files:

```bash
tar -xvzf binsider-*.tar.gz
```

3. Enter the folder and run the binary:

```bash
cd "binsider-${version}"
./binsider
```

4. Move binary to `/usr/local/bin/` (optional).
