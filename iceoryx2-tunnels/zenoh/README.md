# iceoryx2-tunnels-zenoh

> [!CAUTION]
> The implementation is currently in a very early development stage.
> Not all functionality is implemented. The available functionality may not be
> optimal.
>
> If encountering problems, please create an issue so we can converge to
> stability and robustness as soon as possible!

A tunnel utilizing the [`zenoh`](https://github.com/eclipse-zenoh/zenoh)
network middleware to bridge communication between `iceoryx2` instances on
different hosts.

## Usage

1. Install the latest CLI:
    ```console
    git clone git@github.com:eclipse-iceoryx/iceoryx2.git
    cd iceoryx2
    cargo install --path ./iceoryx2-cli
    ```
1. Launch the tunnel via CLI:
    ```console
    iox2 tunnel --help # See available options
    iox2 tunnel zenoh # Run with default options
    ```
1. Use `iceoryx2` as normal
    * The tunnel will periodically to discover services and propagate
      payloads between hosts
