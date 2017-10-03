# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).


## [0.2.1] - 2017-10-03

### Added
* Added [`std::error::Error`](https://doc.rust-lang.org/stable/std/error/trait.Error.html) implementation for [`untrustended::Error`](https://docs.rs/untrustended/0.2.0/untrustended/enum.Error.html) ([#2](https://github.com/oherrala/untrustended/pull/3)).
* Added support for 128-bit integers. Support can be enabled with `i128` feature ([commit](https://github.com/oherrala/untrustended/commit/f97bc73ea539ec04988bab806f0a252981905bda)).

## [0.2.0] - 2017-09-23

### Changed
* Bump [untrusted](https://crates.io/crates/untrusted) to version 0.6.0.
* Rename `read_bytes()` to `read_bytes_less_safe()` ([commit](https://github.com/oherrala/untrustended/commit/8eaae8b008bfb831a7257dd66c58026254a54c9d)).


## [0.1.0] - 2017-08-01

* Things happened. Code appeared.
