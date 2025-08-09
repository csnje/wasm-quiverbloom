# wasm-quiverbloom

## About

An implementation in **Rust** **WebAssembly** and **JavaScript** of several visually striking algorithms designed by [yuruyurau](https://x.com/yuruyurau) .

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

Access via a web browser at [http://localhost:8000](http://localhost:8000).
