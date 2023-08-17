# faststr

[![Crates.io](https://img.shields.io/crates/v/faststr)](https://crates.io/crates/faststr)
[![Documentation](https://docs.rs/faststr/badge.svg)](https://docs.rs/faststr)
[![Website](https://img.shields.io/website?up_message=cloudwego&url=https%3A%2F%2Fwww.cloudwego.io%2F)](https://www.cloudwego.io/)
[![License](https://img.shields.io/crates/l/faststr)](#license)
[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/volo-rs/faststr/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/volo-rs/faststr/actions

`faststr` is a string library that try to avoid the cost of clone.

## Why we need it?

In Rust, the String type is commonly used, but it has the following problems:

1. In many scenarios in asynchronous Rust, we cannot determine when a String is dropped. For example, when we send a String through RPC/HTTP, we cannot explicitly mark the lifetime, thus we must clone it;
2. Rust's asynchronous ecosystem is mainly based on Tokio, with network programming largely relying on bytes::Bytes. We can take advantage of Bytes to avoid cloning Strings, while better integrating with the Bytes ecosystem;
3. Even in purely synchronous code, when the code is complex enough, marking the lifetime can greatly affect code readability and maintainability. In business development experience, there will often be multiple Strings from different sources combined into a single Struct for processing. In such situations, it's almost impossible to avoid cloning using lifetimes;
4. Cloning a String is quite costly;

Therefore, we have created the `FastStr` type. By sacrificing immutability, we can avoid the overhead of cloning Strings and better integrate with Rust's asynchronous, microservice, and network programming ecosystems.

## When should I use it?

1. When you need to send a String through RPC/HTTP;
2. When you read a String from a file or database or config;
3. Everywhere when you don't need to mutate the String anymore;

## How to migrate to `FastStr`?

`FastStr` implements `From` trait for various types, so you can easily migrate to `FastStr` by replacing `String` with `FastStr` and adding `.into()`.

For example, if your API is something like this:

```rust
fn need_a_string(s: String)
```

You may change it to:

```rust
fn need_a_string<S: Into<FastStr>>(s: S)
```

This will not be a break change for users.

## Features

- `serde`: Enable serde support.
- `serde-unsafe`: Enable serde support with utf8 validation disabled.
- `redis`: Enable redis support.
- `redis-unsafe`: Enable redis support with utf8 validation disabled.

## Benchmark

```bash
$ cargo bench
```

### M1Max

```
empty faststr           time:   [19.315 ns 19.345 ns 19.377 ns]

empty string            time:   [2.2097 ns 2.2145 ns 2.2194 ns]

static faststr          time:   [19.483 ns 19.598 ns 19.739 ns]

inline faststr          time:   [20.447 ns 20.476 ns 20.507 ns]

string hello world      time:   [17.215 ns 17.239 ns 17.263 ns]

512B faststr            time:   [23.883 ns 23.922 ns 23.965 ns]

512B string             time:   [50.733 ns 51.360 ns 52.041 ns]

4096B faststr           time:   [23.893 ns 23.959 ns 24.033 ns]

4096B string            time:   [78.323 ns 79.565 ns 80.830 ns]

16384B faststr          time:   [23.829 ns 23.885 ns 23.952 ns]

16384B string           time:   [395.83 ns 402.46 ns 408.51 ns]

65536B faststr          time:   [23.934 ns 24.002 ns 24.071 ns]

65536B string           time:   [1.3142 µs 1.3377 µs 1.3606 µs]

524288B faststr         time:   [23.881 ns 23.926 ns 23.976 ns]

524288B string          time:   [8.8109 µs 8.8577 µs 8.9024 µs]

1048576B faststr        time:   [23.968 ns 24.032 ns 24.094 ns]

1048576B string         time:   [18.424 µs 18.534 µs 18.646 µs]
```

### AMD EPYC 7Y83

```
empty faststr           time:   [42.724 ns 42.728 ns 42.732 ns]

empty string            time:   [4.6490 ns 4.6494 ns 4.6499 ns]

static faststr          time:   [42.519 ns 42.525 ns 42.532 ns]

inline faststr          time:   [43.446 ns 43.450 ns 43.454 ns]

string hello world      time:   [12.385 ns 12.385 ns 12.387 ns]

512B faststr            time:   [42.232 ns 42.238 ns 42.244 ns]

512B string             time:   [15.822 ns 15.846 ns 15.894 ns]

4096B faststr           time:   [41.741 ns 41.918 ns 42.069 ns]

4096B string            time:   [84.492 ns 84.668 ns 84.839 ns]

16384B faststr          time:   [42.245 ns 42.250 ns 42.255 ns]

16384B string           time:   [225.36 ns 225.42 ns 225.47 ns]

65536B faststr          time:   [41.987 ns 42.087 ns 42.166 ns]

65536B string           time:   [1.3212 µs 1.3215 µs 1.3219 µs]

524288B faststr         time:   [42.272 ns 42.277 ns 42.283 ns]

524288B string          time:   [14.373 µs 14.380 µs 14.388 µs]

1048576B faststr        time:   [42.279 ns 42.287 ns 42.295 ns]

1048576B string         time:   [27.995 µs 28.015 µs 28.038 µs]
```

## Related Projects

- [Volo][Volo]: Rust RPC framework with high-performance and strong-extensibility for building micro-services.
- [Motore][Motore]: Middleware abstraction layer powered by GAT.
- [Pilota][Pilota]: A thrift and protobuf implementation in pure rust with high performance and extensibility.
- [Metainfo][Metainfo]: Transmissing metainfo across components.

## Contributing

See [CONTRIBUTING.md](https://github.com/volo-rs/faststr/blob/main/CONTRIBUTING.md) for more information.

All contributions are welcomed!

## License

`faststr` is dual-licensed under the MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](https://github.com/volo-rs/faststr/blob/main/LICENSE-MIT) and [LICENSE-APACHE](https://github.com/volo-rs/faststr/blob/main/LICENSE-APACHE) for details.

## Credits

`faststr` copied and used some code from [`smol_str`](https://github.com/rust-analyzer/smol_str), which is also licensed under the MIT license and the Apache License (Version 2.0).

We really appreciate the work of `smol_str` team.

## Community

- Email: [volo@cloudwego.io](mailto:volo@cloudwego.io)
- How to become a member: [COMMUNITY MEMBERSHIP](https://github.com/cloudwego/community/blob/main/COMMUNITY_MEMBERSHIP.md)
- Issues: [Issues](https://github.com/volo-rs/faststr/issues)
- Feishu: Scan the QR code below with [Feishu](https://www.feishu.cn/) or [click this link](https://applink.feishu.cn/client/chat/chatter/add_by_link?link_token=b34v5470-8e4d-4c7d-bf50-8b2917af026b) to join our CloudWeGo Volo user group.

  <img src="https://github.com/volo-rs/faststr/raw/main/.github/assets/volo-feishu-user-group.png" alt="Volo user group" width="50%" height="50%" />

[Volo]: https://github.com/cloudwego/volo
[Motore]: https://github.com/cloudwego/motore
[Pilota]: https://github.com/cloudwego/pilota
[Metainfo]: https://github.com/cloudwego/metainfo
