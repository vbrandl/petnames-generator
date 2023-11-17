# Petnames Generator

[![Hits-of-Code](https://hitsofcode.com/github/vbrandl/petnames-generator)](https://hitsofcode.com/github/vbrandl/petnames-generator/view)
[![CI](https://github.com/vbrandl/petnames-generator/actions/workflows/ci.yml/badge.svg)](https://github.com/vbrandl/petnames-generator/actions/workflows/ci.yml)
[![dependency status](https://deps.rs/repo/github/vbrandl/petnames-generator/status.svg)](https://deps.rs/repo/github/vbrandl/petnames-generator)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/vbrandl/petnames-generator/blob/master/LICENSE)

**Note**: Since fly.io changed its payment plans and wants payment information to deploy new versions of this project.
Since this was just to experiment, I don't really care but this should be a reminder to not depend on free offerings since they *will* change and not stay free forever.
In general, fly.io was to unreliable for my preference (random errors when deploying, slow or no support at all, ...).
I will not use it for future projects, but it was fun to play around.

A webservice to generate human readable random names using the [`petname`](https://crates.io/crates/petname) crate.
It was created mostly to play arount with the [`axum`](https://crates.io/crates/axum) crate and [fly.io](https://fly.io).

The current release is available online via [petnames.fly.dev](https://petnames.fly.dev/).

## License

`petnames-generator` is licensed under the MIT License ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
