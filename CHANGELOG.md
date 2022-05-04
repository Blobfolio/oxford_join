# Changelog

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
