# SYserve

**S**imple **Y**ew **Serve**r made with [`actix_web`](https://actix.rs/).

Implemented from the [actix example server in the yew-router repo](https://github.com/yewstack/yew_router/tree/master/examples/servers)
with a few changes.

## Usage

```{bash}
$ syserve /path/to/dist/dir
```

where the `dist/` dir contains an `index.html` file which loads the WASM file.
