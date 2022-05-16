# procfile

[![Crate Version](https://img.shields.io/crates/v/procfile)](https://crates.io/crates/procfile)
[![Documentation](https://docs.rs/procfile/badge.svg)](https://docs.rs/procfile)
[![License](https://img.shields.io/crates/l/procfile.svg)](./LICENSE-APACHE)
[![Continuous Delivery](https://github.com/devtomio/procfile/actions/workflows/continuous-delivery.yml/badge.svg)](https://github.com/devtomio/procfile/actions/workflows/continuous-delivery.yml)
[![Continuous Integration](https://github.com/devtomio/procfile/actions/workflows/continuous-integration.yml/badge.svg)](https://github.com/devtomio/procfile/actions/workflows/continuous-integration.yml)

**Procfile parser for Rust**

## Example Usage

> Cargo.toml

```toml
[dependencies]
procfile = "0.1"
```

The code:

```rs
use procfile;

fn main() {
    let procfile = "web: node hello-world.js --verbose";
    let parsed = procfile::parse(procfile).expect("Failed parsing procfile");
    let web = parsed.get("web").expect("Failed getting web process");

    println!("{:?}", web);
}
```

## Cargo Features

- `rayon` - Enables [rayon](https://github.com/rayon-rs/rayon) support (enabled by default)
- `serde` - Enables [serde](https://serde.rs) support

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://tomio.fun/"><img src="https://avatars.githubusercontent.com/u/75403863?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Tomio</b></sub></a><br /><a href="https://github.com/devtomio/procfile/commits?author=devtomio" title="Code">💻</a> <a href="https://github.com/devtomio/procfile/commits?author=devtomio" title="Documentation">📖</a> <a href="#example-devtomio" title="Examples">💡</a> <a href="#infra-devtomio" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-devtomio" title="Maintenance">🚧</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
