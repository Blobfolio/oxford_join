# Oxford Join

[![docs.rs](https://img.shields.io/docsrs/oxford_join.svg?style=flat-square&label=docs.rs)](https://docs.rs/oxford_join/)
[![changelog](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/oxford_join/blob/master/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/oxford_join.svg?style=flat-square&label=crates.io)](https://crates.io/crates/oxford_join)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/oxford_join/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/oxford_join/actions)<br>
[![deps.rs](https://deps.rs/crate/oxford_join/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/oxford_join/)
[![no_std](https://img.shields.io/badge/env-%23![no_std]-00abc0?style=flat-square)](https://docs.rust-embedded.org/book/intro/no-std.html)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/oxford_join/issues)

This crate provides two joining macros for `format!`, `println!`, etc., formatting contexts:
* `join` combines the values of an iterator with the specified glue;
* `oxford_join` combines the values of an iterator with [Oxford commas](https://en.wikipedia.org/wiki/Serial_comma) and a conjunction;

```rust
use oxford_join::{join, oxford_join};

// Straight joins.
assert_eq!(
    format!("{} = 6", join!(" + ", 1_u8..=3_u8)),
    "1 + 2 + 3 = 6",
);

// The good stuff!
assert_eq!(
    oxford_join!("or", ["apples", "bananas"].iter()).to_string(),
    "apples or bananas",
);
assert_eq!(
    oxford_join!("or", ["apples", "bananas", "carrots"].iter()).to_string(),
    "apples, bananas, or carrots",
);
```

Formatting [parameters](https://doc.rust-lang.org/std/fmt/index.html#formatting-parameters) and [traits](https://doc.rust-lang.org/std/fmt/index.html#formatting-traits) are supported by both macros and passed through to each individual value.

```rust
# extern crate alloc;
# use oxford_join::oxford_join;
// Alternate, padding, HEX, oh my!
assert_eq!(
    format!(
        "Checksums: {:#010X}",
        oxford_join!(@UpperHex [30_000, 166, u32::MAX].iter()),
    ),
    "Checksums: 0x00007530, 0x000000A6, and 0xFFFFFFFF",
);
```

See the [documentation](https://docs.rs/oxford_join/) for `join` and `oxford_join` for complete usage details.
