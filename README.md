# build-rs-example
An example project using build.rs to setup build time configuration (env vars) for no_std or ICP like systems that don't have file system access.

## Setup
This project reads a `.env` file to determine the `ENV` to build to.  It matches the value of `ENV` to one of the `enum Profile` variants.
To update values change the values in the key-value pair maps inside `./config.json`.
To change the value compiled and available to the program update the value of `ENV` to match one of the variants in `./config.json`.

## Build and Run
```bash
cargo build
cargo run
```