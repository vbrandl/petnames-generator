# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies
- Bump `serde` from 1.0.192 to 1.0.193 ([#193](https://github.com/vbrandl/petnames-generator/pull/193))
- Bump `tokio` from 1.34.0 to 1.35.1 ([#195](https://github.com/vbrandl/petnames-generator/pull/195), [#198](https://github.com/vbrandl/petnames-generator/pull/198))
- Bump `metrics-exporter-prometheus` from 0.12.1 to 0.13.0 ([#196](https://github.com/vbrandl/petnames-generator/pull/196), [#201](https://github.com/vbrandl/petnames-generator/pull/201))
- Bump `anyhow` from 1.0.75 to 1.0.77 ([#199](https://github.com/vbrandl/petnames-generator/pull/199), [#202](https://github.com/vbrandl/petnames-generator/pull/202))
- Bump `metrics` from 0.21.1 to 0.22.0 ([#200](https://github.com/vbrandl/petnames-generator/pull/200))
- Bump `axum` from 0.7.3 to 0.7.4 ([#206](https://github.com/vbrandl/petnames-generator/pull/206))
- Bump `tower-http` from 0.5.0 to 0.5.1 ([#209](https://github.com/vbrandl/petnames-generator/pull/209))
- Bump `axum-extra` from 0.9.1 to 0.9.2 ([#208](https://github.com/vbrandl/petnames-generator/pull/208))
- Bump `vergen` from 8.2.7 to 8.3.0 ([#210](https://github.com/vbrandl/petnames-generator/pull/210))

## [0.18.0] 2023-11-17
### Dependencies
- Bump `hyper` from 0.14.26 to 0.14.27 (#140)
- Bump `tokio` from 1.28.2 to 1.34.0 (#141, #142, [#165](https://github.com/vbrandl/petnames-generator/pull/165), [#166](https://github.com/vbrandl/petnames-generator/pull/166), [#169](https://github.com/vbrandl/petnames-generator/pull/169), [#177](https://github.com/vbrandl/petnames-generator/pull/177), [#185](https://github.com/vbrandl/petnames-generator/pull/185))
- Bump `metrics` from 0.21.0 to 0.21.1 (#144)
- Bump `serde` from 1.0.164 to 1.0.192 (#145, #147, #148, [#154](https://github.com/vbrandl/petnames-generator/pull/154), [#156](https://github.com/vbrandl/petnames-generator/pull/156), [#157](https://github.com/vbrandl/petnames-generator/pull/157), [#159](https://github.com/vbrandl/petnames-generator/pull/159), [#160](https://github.com/vbrandl/petnames-generator/pull/160), [#161](https://github.com/vbrandl/petnames-generator/pull/161), [#163](https://github.com/vbrandl/petnames-generator/pull/163), [#164](https://github.com/vbrandl/petnames-generator/pull/164), [#171](https://github.com/vbrandl/petnames-generator/pull/171), [#178](https://github.com/vbrandl/petnames-generator/pull/178), [#182](https://github.com/vbrandl/petnames-generator/pull/182), [#184](https://github.com/vbrandl/petnames-generator/pull/184))
- Bump `vergen` from 8.2.1 to 8.2.6 (#143, #146, [#174](https://github.com/vbrandl/petnames-generator/pull/174), [#183](https://github.com/vbrandl/petnames-generator/pull/183))
- Bump `anyhow` from 1.0.71 to 1.0.75 (#149, [#167](https://github.com/vbrandl/petnames-generator/pull/167), [#168](https://github.com/vbrandl/petnames-generator/pull/168))
- Bump `axum` from 0.6.18 to 0.6.20 (#150, [#162](https://github.com/vbrandl/petnames-generator/pull/162))
- Bump `tower-http` from 0.4.1 to 0.4.4 (#151, [#153](https://github.com/vbrandl/petnames-generator/pull/153), [#172](https://github.com/vbrandl/petnames-generator/pull/172))
- Bump `ructe` from 0.16.1 to 0.17.0 ([#155](https://github.com/vbrandl/petnames-generator/pull/155))
- Bump `actions/checkout` from 3 to 4 ([#173](https://github.com/vbrandl/petnames-generator/pull/173))
- Bump `stefanzweifel/git-auto-commit-action` from 4 to 5 ([#176](https://github.com/vbrandl/petnames-generator/pull/176))
- Bump `tracing` from 0.1.37 to 0.1.40 ([#179](https://github.com/vbrandl/petnames-generator/pull/179), [#181](https://github.com/vbrandl/petnames-generator/pull/181))
- Bump `rustix` from 0.37.22 to 0.37.25 ([#180](https://github.com/vbrandl/petnames-generator/pull/180))
- Bump `tracing-subscriber` from 0.3.17 to 0.3.18 ([#187](https://github.com/vbrandl/petnames-generator/pull/187))
- Bump `axum` to 0.7.3 ([#197](https://github.com/vbrandl/petnames-generator/pull/197))


## [0.17.0] 2023-06-21

### Dependencies
- Bump `serde` from 1.0.160 to 1.0.164 (#120, #130, #135)
- Bump `tokio` from 1.28.0 to 1.28.2 (#129, #134)
- Bump `metrics-exporter-prometheus` from 0.12.0 to 0.12.1 (#128)
- Bump `vergen` from 8.1.3 to 8.2.1 (#132, #133)
- Bump `tower-http` from 0.4.0 to 0.4.1 (#138)


## [0.16.0] 2023-05-04

* Updated [`h2`](https://github.com/hyperium/h2) from 0.3.16 to 0.3.17, fixes [SEC#6] ([#105])
* Updated [`hyper`](https://github.com/hyperium/hyper) from 0.14.25 to 0.14.26 ([#106])
* Updated [`metrics`](https://github.com/metrics-rs/metrics) from 0.20.1 to 0.21.0 ([#107])
* Updated [`metrics-exporter-prometheus`](https://github.com/metrics-rs/metrics) from 0.11.0 to 0.12.0 ([#108])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.15 to 0.6.16 ([#109])
* Updated [`tracing-subscriber`](https://github.com/tokio-rs/tracing) from 0.3.16 to 0.3.17 ([#110])
* Updated [`tracing`](https://github.com/tokio-rs/tracing) from 0.1.37 to 0.1.38 ([#111])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.16 to 0.6.17 ([#112])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.27.0 to 1.28.0 ([#113])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 8.1.1 to 8.1.3 ([#115])
* Downgrade yanked [`tracing`](https://github.com/tokio-rs/tracing) 0.1.38 to 0.1.37 ([#118])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.17 to 0.6.18 ([#116])
* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.70 to 1.0.71 ([#117])

[#105]: https://github.com/vbrandl/petnames-generator/pull/105
[#106]: https://github.com/vbrandl/petnames-generator/pull/106
[#107]: https://github.com/vbrandl/petnames-generator/pull/107
[#108]: https://github.com/vbrandl/petnames-generator/pull/108
[#109]: https://github.com/vbrandl/petnames-generator/pull/109
[#110]: https://github.com/vbrandl/petnames-generator/pull/110
[#111]: https://github.com/vbrandl/petnames-generator/pull/111
[#112]: https://github.com/vbrandl/petnames-generator/pull/112
[#113]: https://github.com/vbrandl/petnames-generator/pull/113
[#115]: https://github.com/vbrandl/petnames-generator/pull/115
[#118]: https://github.com/vbrandl/petnames-generator/pull/118
[#116]: https://github.com/vbrandl/petnames-generator/pull/116
[#117]: https://github.com/vbrandl/petnames-generator/pull/117

[SEC#6]: https://github.com/vbrandl/petnames-generator/security/dependabot/6

## [0.15.0] 2023-04-13

### Changes

* Enable metrics endpoint again ([#103])

### Dependencies

* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.69 to 1.0.70 ([#90])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.156 to 1.0.158 ([#92])
* Updated [`mime`](https://github.com/hyperium/mime) from 0.3.16 to 0.3.17 ([#93])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.11 to 0.6.12 ([#94])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.26.0 to 1.27.0 ([#96])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.158 to 1.0.159 ([#97])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.159 to 1.0.160 ([#101])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.12 to 0.6.15 ([#102])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.5.1 to 8.1.1 ([#99])

[#90]: https://github.com/vbrandl/petnames-generator/pull/90
[#92]: https://github.com/vbrandl/petnames-generator/pull/92
[#93]: https://github.com/vbrandl/petnames-generator/pull/93
[#94]: https://github.com/vbrandl/petnames-generator/pull/94
[#96]: https://github.com/vbrandl/petnames-generator/pull/96
[#97]: https://github.com/vbrandl/petnames-generator/pull/97
[#101]: https://github.com/vbrandl/petnames-generator/pull/101
[#102]: https://github.com/vbrandl/petnames-generator/pull/102
[#99]: https://github.com/vbrandl/petnames-generator/pull/99
[#103]: https://github.com/vbrandl/petnames-generator/pull/103

## [0.14.0] 2023-03-16

### Dependencies

* Updated [`hyper`](https://github.com/hyperium/hyper) from 0.14.24 to 0.14.25 ([#86])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.10 to 0.6.11 ([#87])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.152 to 1.0.156 ([#88])

[#86]: https://github.com/vbrandl/petnames-generator/pull/86
[#87]: https://github.com/vbrandl/petnames-generator/pull/87
[#88]: https://github.com/vbrandl/petnames-generator/pull/88


## [0.13.0] 2023-03-06

### Dependencies

* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.66 to 1.0.68 ([#52])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.0 to 0.6.1 ([#53])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.4.3 to 7.4.4 ([#54])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.151 to 1.0.152 ([#55])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.23.0 to 1.23.1 ([#56])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.23.1 to 1.24.0 ([#57])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.4.4 to 7.5.0 ([#58])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.24.0 to 1.24.1 ([#60])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.1 to 0.6.2 ([#61])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.24.1 to 1.24.2 ([#62])
* Updated [`bumpalo`](https://github.com/fitzgen/bumpalo) from 3.11.0 to 3.12.0 ([#63])
* Updated [`libgit2-sys`](https://github.com/rust-lang/git2-rs) from 0.14.0+1.5.0 to 0.14.2+1.5.1 ([#64])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.2 to 0.6.4 ([#67])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.24.2 to 1.25.0 ([#68])
* Updated [`ructe`](https://github.com/kaj/ructe) from 0.15.0 to 0.16.1 ([#69])
* Updated transitive dependencies in `Cargo.lock` ([#70])
* Updated [`hyper`](https://github.com/hyperium/hyper) from 0.14.23 to 0.14.24 ([#71])
* Updated [`anyhow`](https://github.com/dtolnay/anyhow) from 1.0.68 to 1.0.69 ([#72])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.5.0 to 7.5.1 ([#73])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.4 to 0.6.6 ([#74])
* Updated [`tower-http`](https://github.com/tower-rs/tower-http) from 0.3.5 to 0.4.0 ([#77])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.25.0 to 1.26.0 ([#79])
* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.6.6 to 0.6.10 ([#80])


[#52]: https://github.com/vbrandl/petnames-generator/pull/52
[#53]: https://github.com/vbrandl/petnames-generator/pull/53
[#54]: https://github.com/vbrandl/petnames-generator/pull/54
[#55]: https://github.com/vbrandl/petnames-generator/pull/55
[#56]: https://github.com/vbrandl/petnames-generator/pull/56
[#57]: https://github.com/vbrandl/petnames-generator/pull/57
[#58]: https://github.com/vbrandl/petnames-generator/pull/58
[#60]: https://github.com/vbrandl/petnames-generator/pull/60
[#61]: https://github.com/vbrandl/petnames-generator/pull/61
[#62]: https://github.com/vbrandl/petnames-generator/pull/62
[#63]: https://github.com/vbrandl/petnames-generator/pull/63
[#64]: https://github.com/vbrandl/petnames-generator/pull/64
[#67]: https://github.com/vbrandl/petnames-generator/pull/67
[#68]: https://github.com/vbrandl/petnames-generator/pull/68
[#69]: https://github.com/vbrandl/petnames-generator/pull/69
[#70]: https://github.com/vbrandl/petnames-generator/pull/70
[#71]: https://github.com/vbrandl/petnames-generator/pull/71
[#72]: https://github.com/vbrandl/petnames-generator/pull/72
[#73]: https://github.com/vbrandl/petnames-generator/pull/73
[#74]: https://github.com/vbrandl/petnames-generator/pull/74
[#77]: https://github.com/vbrandl/petnames-generator/pull/77
[#79]: https://github.com/vbrandl/petnames-generator/pull/79
[#80]: https://github.com/vbrandl/petnames-generator/pull/80

## [0.12.1] 2022-12-18

* Disable metrics server, to maybe fix the Fly.io deployment ([#51])

[#51]: https://github.com/vbrandl/petnames-generator/pull/51

## [0.12.0] 2022-12-18

### Dependencies

* Updated [`axum`](https://github.com/tokio-rs/axum) from 0.5.17 to 0.6.0 ([#43])
* Updated [`vergen`](https://github.com/rustyhorde/vergen) from 7.4.2 to 7.4.3 ([#44])
* Updated [`tower-http`](https://github.com/tower-rs/tower-http) from 0.3.4 to 0.3.5 ([#45])
* Updated [`tokio`](https://github.com/tokio-rs/tokio) from 1.22.0 to 1.23.0 ([#47])
* Updated [`serde`](https://github.com/serde-rs/serde) from 1.0.147 to 1.0.151 ([#49])

[#43]: https://github.com/vbrandl/petnames-generator/pull/43
[#44]: https://github.com/vbrandl/petnames-generator/pull/44
[#45]: https://github.com/vbrandl/petnames-generator/pull/45
[#47]: https://github.com/vbrandl/petnames-generator/pull/47
[#49]: https://github.com/vbrandl/petnames-generator/pull/49


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
