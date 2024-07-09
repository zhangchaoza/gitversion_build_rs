# gitversion_build_rs

Build crate with versions by gitversion

## usage

Install [gitversion cli](https://gitversion.net/docs/usage/cli/installation) first.

Add `gitversion_build` as build-dependencies

```
cargo add gitversion_build --build
```

Add the code to your `build.rs`

```rust
gitversion_build::inject_version().unwrap();
```

Then you can get `CARGO_PKG_VERSION` `CARGO_PKG_VERSION_MAJOR` `CARGO_PKG_VERSION_MINOR` `CARGO_PKG_VERSION_PATCH` `CARGO_PKG_VERSION_PRE` by using `env!` in your code.

A example at [example/demo](https://github.com/zhangchaoza/gitversion_build_rs/tree/main/example/demo) .