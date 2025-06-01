# Oxford Join

[![docs.rs](https://img.shields.io/docsrs/oxford_join.svg?style=flat-square&label=docs.rs)](https://docs.rs/oxford_join/)
[![changelog](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=crates.io)](https://crates.io/crates/oxford_join)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/oxford_join/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/oxford_join/actions)
[![deps.rs](https://deps.rs/crate/oxford_join/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/oxford_join/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/oxford_join/issues)

Join a slice of strings with [Oxford Commas](https://en.wikipedia.org/wiki/Serial_comma) inserted as necessary, using the `Conjunction` of your choice.

(You know, as it should be. Haha.)

The return formatting depends on the size of the set:

```
0: ""
1: "first"
2: "first <CONJUNCTION> last"
n: "first, second, …, <CONJUNCTION> last"
```

This crate is `#![no_std]`-compatible.

## Examples

The magic is accomplished with the `OxfordJoin` trait. Import that, and most
slice-y things holding `AsRef<str>` will inherit the `OxfordJoin::oxford_join`
method for joining.

```rust
use oxford_join::{Conjunction, OxfordJoin};

let set = ["Apples", "Oranges"];
assert_eq!(set.oxford_join(Conjunction::And), "Apples and Oranges");

let set = ["Apples", "Oranges", "Bananas"];
assert_eq!(set.oxford_join(Conjunction::And), "Apples, Oranges, and Bananas");

// There are also shorthand methods for and, or, and_or, and nor, allowing you
// to skip the Conjunction enum entirely.
assert_eq!(set.oxford_and(), "Apples, Oranges, and Bananas");
assert_eq!(set.oxford_and_or(), "Apples, Oranges, and/or Bananas");
assert_eq!(set.oxford_nor(), "Apples, Oranges, nor Bananas");
assert_eq!(set.oxford_or(), "Apples, Oranges, or Bananas");
```

That's all, folks!



## Installation

Add `oxford_join` to your `dependencies` in `Cargo.toml`, like:

```toml
[dependencies]
oxford_join = "0.6.*"
```
