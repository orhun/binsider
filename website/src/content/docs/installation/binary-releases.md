---
title: Binary releases
sidebar:
  order: 4
---

See the available binaries for different operating systems/architectures from the [releases page](https://github.com/orhun/binsider/releases).

:::note

- Only the `x86_64-unknown-linux-gnu` target is provided for now.
- Release tarballs are signed with the following PGP key: [0C9E792408F77819866E47FA85EF5848473D7F88](https://keyserver.ubuntu.com/pks/lookup?search=0x85EF5848473D7F88&op=vindex)

:::

1. Download the binary from [releases](https://github.com/orhun/binsider/releases)

```bash
VERSION="0.1.0"
wget "https://github.com/orhun/binsider/releases/download/v${VERSION}/binsider-${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
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
