# SYserve

[![Crates.io](https://img.shields.io/crates/v/syserve.svg)](https://crates.io/crates/syserve)
[![Crates.io](https://img.shields.io/crates/l/syserve.svg)](./LICENSE)

**S**imple **Y**ew **Serve**r (inspired by [devserver](https://github.com/kettle11/devserver))

## Usage

```bash
syserve -d /path/to/dist/dir
```

where the `dist/` dir contains an `index.html` file which loads the WASM file.
