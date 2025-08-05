# Installation Instructions

## Install Dependencies

Since iceoryx2 is written in Rust we need to install that first. We recommend
the [official approach](https://www.rust-lang.org/tools/install).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Additionally, install poetry to simplify managing the Python virtual
environment:

```sh
curl -sSL https://install.python-poetry.org | python3 -
```

Then you can set up a virtual environment and install all dependencies using

```sh
cd $(git rev-parse --show-toplevel)

poetry --project iceoryx2-ffi/python install
```

## Build

Compile iceoryx2 and the Python language bindings for development by calling:

```sh
cd $(git rev-parse --show-toplevel)

poetry --project iceoryx2-ffi/python run maturin develop --manifest-path iceoryx2-ffi/python/Cargo.toml --target-dir target/ffi/python
```

The language bindings will be then available for use inside the virtual
environment.

## Running Examples

You can then run any Python example from the virtual environment managed by
poetry:

```sh
cd $(git rev-parse --show-toplevel)

poetry --project iceoryx2-ffi/python run python examples/python/publish_subscribe/publisher.py
```
