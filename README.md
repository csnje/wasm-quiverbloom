# wasm-quiverbloom

## About

An implementation of several visually striking algorithms originally created by [yuruyurau](https://x.com/yuruyurau) in **Rust** **WebAssembly**.

![Image of output](./images/output.png)

## Prerequisites

A [**Rust**](https://www.rust-lang.org/) installation.

## Build

```sh
cargo build --target=wasm32-unknown-unknown --release
```

## Run

Some options to serve the application include:
```bash
# Python 3.x
python3 -m http.server
# Python 2.x
python -m SimpleHTTPServer
# JDK 18 or later
jwebserver
```

Access via a browser at [http://localhost:8000](http://localhost:8000).
