# syserve

[![Crates.io](https://img.shields.io/crates/v/syserve.svg)](https://crates.io/crates/syserve)
[![Crates.io](https://img.shields.io/crates/l/syserve.svg)](./LICENSE)

**S**imple **Y**ew **Serve**r.

Inspired from [devserver](https://github.com/kettle11/devserver) and the final project from the [rust book](https://doc.rust-lang.org/stable/book/) as a simple, minimal development server tailored to yew and yew router apps.

This crate does not utilize any large server libraries like actix or rocket and is written using `std::net` with the `httparse` and `threadpool` libraries.
Syserve is meant to be a minimal but featureful development server with HTTPS support (intended) and customization over file serving.

**This should not be used in production.**

## Installation

```bash
$ cargo install syserve
```

Should install in less than a minute.

## Usage

```bash
$ syserve -h
Usage:
  syserve [OPTIONS]

Minimal server for yew and yew-router apps

Optional arguments:
  -h,--help             Show this help message and exit
  -d,--dir DIR          Directory to serve (must contain index.html). Default: current directory
  -i,--index INDEX      Index file to be served. Default: index.html
  -n,--404,--not-found NOT_FOUND
                        File to be served in case of 404
  -a,--addr ADDR        Address at which files are served. Default: localhost
  -p,--port PORT        Port at which files are served. Default: 7878
```

# Examples

```bash
$ syserve -d /path/to/dist/dir
```

where the `dist/` dir contains an `index.html` file which loads the WASM file. Served at `http://localhost:7878`
