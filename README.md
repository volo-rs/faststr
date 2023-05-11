# faststr

[![Crates.io](https://img.shields.io/crates/v/faststr)](https://crates.io/crates/faststr)
[![Documentation](https://docs.rs/faststr/badge.svg)](https://docs.rs/faststr)
[![Website](https://img.shields.io/website?up_message=cloudwego&url=https%3A%2F%2Fwww.cloudwego.io%2F)](https://www.cloudwego.io/)
[![License](https://img.shields.io/crates/l/faststr)](#license)
[![Build Status][actions-badge]][actions-url]

[actions-badge]: https://github.com/volo-rs/faststr/actions/workflows/ci.yaml/badge.svg
[actions-url]: https://github.com/volo-rs/faststr/actions

`faststr` is a string library that try to avoid the cost of clone.

## Related Projects

- [Volo][Volo]: Rust RPC framework with high-performance and strong-extensibility for building micro-services.
- [Motore][Motore]: Middleware abstraction layer powered by GAT.
- [Pilota][Pilota]: A thrift and protobuf implementation in pure rust with high performance and extensibility.
- [Metainfo][Metainfo]: Transmissing metainfo across components.

## Contributing

See [CONTRIBUTING.md](https://github.com/volo-rs/faststr/blob/main/CONTRIBUTING.md) for more information.

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
