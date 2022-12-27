# Changelog



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
