# Changelog



## [0.2.10](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.10) - 2024-09-05

### Changed

* Miscellaneous code cleanup and lints
* Bump `brunch` to `0.6`



## [0.2.9](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.9) - 2023-10-05

### Changed

* Minor code lints and cleanup
* Add `no-std` tests to CI



## [0.2.8](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.8) - 2023-06-01

This release improves unit test coverage, but has no particular user-facing changes.



## [0.2.7](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.7) - 2023-01-26

### Changed

* Bump brunch `0.4`



## [0.2.6](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.6) - 2022-12-27

### Changed

* Minor performance improvement for slice joins
* Improve badge consistency (docs)



## [0.2.5](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.5) - 2022-09-11

### New

* impl `OxfordJoin` for `BTreeSet`
* impl `OxfordJoin` for `BTreeMap` (values)
* `Conjunction::oxford_join`

### Changed

* Minor performance improvements



## [0.2.4](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.4) - 2022-08-11

### Changed

* Bump MSRV 1.62



## [0.2.3](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.3) - 2022-04-30

### Changed

* Make crate `#![no_std]` w/o any feature gates



## [0.2.2](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.2) - 2022-04-05

### Changed

* Remove redundant `OxfordJoin` impls on `Vec<T>` and `Box<[T]>`
* Optimize joins of two (e.g. `[T; 2]`)

### Fixed

* Correct `String` capacity calculation for unspecialized slice joins (two extra bytes were being reserved!)



## [0.2.1](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.1) - 2022-01-13

### Added

* `Conjunction::is_empty`
* impl `Hash`, `Borrow<str>` for `Conjunction`



## [0.2.0](https://github.com/Blobfolio/oxford_join/releases/tag/v0.2.0) - 2021-10-21

### Added

* This changelog! Haha.

### Changed

* Use Rust edition 2021.
