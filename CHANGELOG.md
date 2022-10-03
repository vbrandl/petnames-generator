# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


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
