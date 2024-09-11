---
title: Docker images
sidebar:
  order: 6
---

Docker images are available at:

- [Docker Hub](https://hub.docker.com/r/orhunp/binsider)
- [GitHub Container Registry](https://github.com/orhun/binsider/pkgs/container/binsider)

You can use the following command to run the latest version of `binsider` in a container:

```bash
docker run --rm -it "orhunp/binsider:${TAG:-latest}"
```

To analyze a custom binary via mounting a volume:

```bash
docker run --rm -it -v "custom:/app/custom:rw" "orhunp/binsider:${TAG:-latest}" custom
```
