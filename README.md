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

### AARCH64

#### M3Max

```
empty faststr           time:   [2.0188 ns 2.0271 ns 2.0356 ns]

empty string            time:   [2.1306 ns 2.1333 ns 2.1365 ns]

static faststr          time:   [2.0458 ns 2.0589 ns 2.0709 ns]

inline faststr          time:   [2.2270 ns 2.2332 ns 2.2399 ns]

string hello world      time:   [12.553 ns 12.575 ns 12.597 ns]

512B faststr            time:   [3.8373 ns 3.8454 ns 3.8540 ns]

512B string             time:   [36.895 ns 37.007 ns 37.121 ns]

4096B faststr           time:   [3.8205 ns 3.8260 ns 3.8317 ns]

4096B string            time:   [55.275 ns 55.355 ns 55.446 ns]

16384B faststr          time:   [3.8191 ns 3.8246 ns 3.8306 ns]

16384B string           time:   [338.18 ns 352.36 ns 365.02 ns]

65536B faststr          time:   [3.8169 ns 3.8221 ns 3.8277 ns]

65536B string           time:   [662.52 ns 663.75 ns 664.96 ns]

524288B faststr         time:   [3.8140 ns 3.8178 ns 3.8219 ns]

524288B string          time:   [6.2681 µs 6.2755 µs 6.2827 µs]

1048576B faststr        time:   [3.8235 ns 3.8290 ns 3.8348 ns]

1048576B string         time:   [12.422 µs 12.438 µs 12.453 µs]
```

### amd64

#### AMD EPYC 7Y83

```
empty faststr           time:   [4.3325 ns 4.3330 ns 4.3335 ns]

empty string            time:   [4.6413 ns 4.6422 ns 4.6434 ns]

static faststr          time:   [4.3328 ns 4.3333 ns 4.3339 ns]

inline faststr          time:   [4.6567 ns 4.6580 ns 4.6593 ns]

string hello world      time:   [12.897 ns 12.929 ns 12.954 ns]

512B faststr            time:   [4.4218 ns 4.4253 ns 4.4291 ns]

512B string             time:   [16.087 ns 16.094 ns 16.105 ns]

4096B faststr           time:   [4.4066 ns 4.4099 ns 4.4141 ns]

4096B string            time:   [96.905 ns 97.401 ns 97.879 ns]

16384B faststr          time:   [4.4150 ns 4.4277 ns 4.4414 ns]

16384B string           time:   [229.25 ns 229.30 ns 229.34 ns]

65536B faststr          time:   [4.4562 ns 4.4623 ns 4.4690 ns]

65536B string           time:   [1.3325 µs 1.3328 µs 1.3332 µs]

524288B faststr         time:   [4.4167 ns 4.4240 ns 4.4326 ns]

524288B string          time:   [18.268 µs 18.277 µs 18.287 µs]

1048576B faststr        time:   [4.4275 ns 4.4385 ns 4.4494 ns]

1048576B string         time:   [32.839 µs 33.777 µs 34.554 µs]
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
