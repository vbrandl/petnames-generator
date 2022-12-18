# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.5.17 to 0.6.0 ([#43])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.4.2 to 7.4.3 ([#44])
* Updated [`tower-http`](https://github.com/tower-rs/tower-http) from 0.3.4 to 0.3.5 ([#45])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.22.0 to 1.23.0 ([#47])

[#43]: https://github.com/vbrandl/petnames-generator/pull/43
[#44]: https://github.com/vbrandl/petnames-generator/pull/44
[#45]: https://github.com/vbrandl/petnames-generator/pull/45
[#47]: https://github.com/vbrandl/petnames-generator/pull/47


## [0.11.0] 2022-11-21

### Dependencies

* Updated [`petname`](https://github.com/allenap/rust-petname) from 1.1.2 to 1.1.3 ([#37])
* Updated [`hyper`](https://github.com/hyperium/hyper) from 0.14.22 to 0.14.23 ([#38])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.21.2 to 1.22.0 ([#40])

[#37]: https://github.com/vbrandl/petnames-generator/pull/37
[#38]: https://github.com/vbrandl/petnames-generator/pull/38
[#40]: https://github.com/vbrandl/petnames-generator/pull/40

## [0.10.0] 2022-11-06

### Dependencies

* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.65 to 1.0.66 ([#33])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.5.16 to 0.5.17 ([#34])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.145 to 1.0.147 ([#35])
* Updated [`hyper`](https://github.com/hyperium/hyper) from 0.14.20 to 0.14.22 ([#36])

[#33]: https://github.com/vbrandl/petnames-generator/pull/33
[#34]: https://github.com/vbrandl/petnames-generator/pull/34
[#35]: https://github.com/vbrandl/petnames-generator/pull/35
[#36]: https://github.com/vbrandl/petnames-generator/pull/36


### Changed

* Return Internal Server Error response on rendering errors ([#29])
* Allow optional trailing comma in `render!` macro ([#30])

[#29]: https://github.com/vbrandl/petnames-generator/pull/29
[#30]: https://github.com/vbrandl/petnames-generator/pull/30

## [0.9.0] 2022-10-07

### Changed

* Updated tracing from 0.1.36 to 0.1.37 ([#26])
* Updated tracing-subscriber from 0.3.15 to 0.3.16 ([#27])

### Fixed

* Removed dependency on unmaintained `ansi_term` (RUSTSEC-2021-0139) ([#15])

[#15]: https://github.com/vbrandl/petnames-generator/pull/15
[#26]: https://github.com/vbrandl/petnames-generator/pull/26
[#27]: https://github.com/vbrandl/petnames-generator/pull/27

## [0.8.0] 2022-10-03

### Changed

* Disable metrics for `/metrics` endpoint ([#24])


[#24]: https://github.com/vbrandl/petnames-generator/pull/24


## [0.7.0] 2022-10-03

### Fixed

* [#21]: Clean git repository for deployment image ([#22])


[#21]: https://github.com/vbrandl/petnames-generator/pull/21
[#22]: https://github.com/vbrandl/petnames-generator/pull/22


## [0.6.0] 2022-10-03

### Changed

* Start second server for `/metrics` endpoint. This endpoint should not be publicly available. ([#19])


[#19]: https://github.com/vbrandl/petnames-generator/pull/19


## [0.5.0] 2022-10-03

### Changed

* Reject zero for numeric parameters by using `NonZeroU8` ([#17])

### Fixed

* Ensure uniqueness for generated names. Temporary fix for [#1] ([#17])


[#1]: https://github.com/vbrandl/petnames-generator/pull/1
[#17]: https://github.com/vbrandl/petnames-generator/pull/17


## [0.4.0] 2022-10-02

### Changed

* Error page for bad requests ([#11])
* Use matching layout for bad request and not found error pages ([#11])

### Fixed

* Minor HTML fixes ([#11])


[#11]: https://github.com/vbrandl/petnames-generator/pull/11


## [0.3.0] 2022-10-02

### Changed

* Better layout for generated names ([#7])
* Prefill form with parameters from current query ([#7])
* Use medium wordlist ([#8])

### Fixed

* Proper rejection of invalid queries ([#7])

[#7]: https://github.com/vbrandl/petnames-generator/pull/7
[#8]: https://github.com/vbrandl/petnames-generator/pull/8

## [0.2.0] 2022-10-02

### Changed

* Use large wordlist and improve performance by reusing the random number generator ([#4])

### Fixed

 * Fixed broken testcase for `/metrics` endpoint ([#2], [#3])
 * Shortened the buildscript ([#3])

[#2]: https://github.com/vbrandl/petnames-generator/pull/2
[#3]: https://github.com/vbrandl/petnames-generator/pull/3
[#4]: https://github.com/vbrandl/petnames-generator/pull/4

## [0.1.0] 2022-10-02

### Changed

* Initial version of the webservice
