# af-tools

_wip_: a [Max](https://cycling74.com/products/max) package which is a collection
of various tools.

* [af.path.template](af-path-template/README.md) - opinionated logic for
  transforming paths
* [af.path.walk](af-path-walk/README.md) - alternative filesystem walker which
  honors ignore files

## building

First check/adjust the `PLATFORM_INSTALL_DIR` env in `af-path*/Makefile.toml` to make sure the installation lands in a directory in Max's search path, then:

```sh
cargo make install
```
