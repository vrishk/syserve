//! A simple development server for yew and yew-router apps
//! **S**imple **Y**ew **Serve**r.
//! Inspired from [devserver](https://github.com/kettle11/devserver) and the final project from the [rust book](https://doc.rust-lang.org/stable/book/) as a simple, minimal development server tailored to yew and yew router apps.
//!
//! This crate does not utilize any large server libraries like actix or rocket and is written using `std::net` with the `httparse` and `threadpool` libraries.
//! Syserve is meant to be a minimal but featureful development server with HTTPS support (intended) and customization over file serving.
//!
//! **This should not be used in production.**

pub mod cli;
pub mod server;
pub mod utils;
