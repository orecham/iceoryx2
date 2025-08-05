# iceoryx2-ffi-python

Quick reference for commands relevant for development of the `iceoryx2` Python bindings.

## Install Poetry

```sh
curl -sSL https://install.python-poetry.org | python3 -
```

## Setup Virtual Environment

```sh
cd $(git rev-parse --show-toplevel)

# Install dependencies and create virtual environment
poetry --project iceoryx2-ffi/python install

# (OPTIONAL) Enter the virtual environment - skip the 'poetry run' prefix for all commands
eval $(poetry --project iceoryx2-ffi/python env activate)
```

## Development

```sh
cd $(git rev-parse --show-toplevel)

# Compile PyO3 bindings
poetry --project iceoryx2-ffi/python run maturin develop --manifest-path iceoryx2-ffi/python/Cargo.toml --target-dir target/ffi/python

# Test python bindings
poetry --project iceoryx2-ffi/python run pytest iceoryx2-ffi/python/tests/*

# Run static code analysis
poetry --project iceoryx2-ffi/python run prospector -m -D -T --with-tool mypy -s veryhigh $(pwd)/examples/python
poetry --project iceoryx2-ffi/python run prospector -m -D -T --with-tool mypy -s veryhigh $(pwd)/iceoryx2-ffi/python/tests

# Run formatting: import ordering
poetry --project iceoryx2-ffi/python run isort $(pwd)/examples/python
poetry --project iceoryx2-ffi/python run isort $(pwd)/iceoryx2-ffi/python/tests

# Run formatting
poetry --project iceoryx2-ffi/python run black $(pwd)/examples/python
poetry --project iceoryx2-ffi/python run black $(pwd)/iceoryx2-ffi/python/tests
```

## Run Examples

```sh
cd $(git rev-parse --show-toplevel)

poetry --project iceoryx2-ffi/python run python examples/python/event/listener.py
```
