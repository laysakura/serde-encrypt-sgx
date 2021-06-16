# serde-encrypt-sgx

![MSRV](https://img.shields.io/badge/rustc-1.49+-lightgray.svg)
[![ci](https://github.com/laysakura/serde-encrypt/actions/workflows/ci.yml/badge.svg?branch=main&event=push)](https://github.com/laysakura/serde-encrypt/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/serde-encrypt/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/serde-encrypt/blob/master/LICENSE-APACHE)

[RUST SGX SDK](https://github.com/apache/incubator-teaclave-sgx-sdk) compatible version of [`serde-encrypt` crate](https://github.com/laysakura/serde-encrypt).

This repository only provides SGX-specific implementations.

Refer to [`serde-encrypt` crate](https://github.com/laysakura/serde-encrypt) for documentation, issues, and so on.

## Installation

```toml Cargo.toml
[dependencies]
serde-encrypt-sgx = {tag = "(git tag)", git = "https://github.com/laysakura/serde-encrypt-sgx.git"}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in serde-encrypt by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
