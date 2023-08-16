# Rust String Benchmarks

This repo tries to assess Rust string types.

We currently compare:

Name                                                  | Size     |Heap  | Inline   | `&'static str` | Mutable | Unsafe | Notes
------------------------------------------------------|----------|------|----------|----------------|---------|--------|-----
`String`                                              | 24 bytes | **Y** | \-       | N              | **Y**   | \-     | Universal
`Cow<'static, str>`                                   | 24 bytes | **Y** | \-       | **Y**          | N       | \-     |
[`arcstr`](https://crates.io/crates/arcstr)           | 8 bytes  | ?     | ?        | ?              | ?       | ?  | ?
[`compact_str`](https://crates.io/crates/compact_str) | 24 bytes | **Y** | 24 bytes | N              | **Y**   | **Y** (miri, proptest, fuzz)  | Space optimized for `Option<_>`
[`flexstr`](https://crates.io/crates/flexstr)         | 24 bytes | **Y** | 22 bytes | **Y**          | N       | **Y** (miri) | O(1) clone
[`hipstr`](https://crates.io/crates/hipstr)           | 24 bytes | ?     | ?        | ?              | ?       | ?  | ?
[`imstr`](https://crates.io/crates/imstr)             | 24 bytes | ?     | ?        | ?              | ?       | ?  | ?
[`kstring`](https://crates.io/crates/kstring)         | 24 bytes | **Y** | 15 bytes | **Y**          | N       | Optional (miri, proptest)  | Optional O(1) clone, optional 22 byte small string, Ref/Cow API for preserving `&'static str`
[`smartstring`](https://crates.io/crates/smartstring) | 24 bytes | **Y** | 23 bytes | N              | **Y**   | **Y** (miri, proptest, fuzz)  |
[`smol_str`](https://crates.io/crates/smol_str)       | 24 bytes | **Y** | 22 bytes | N              | N       | **Y** (miri, proptest)  | O(1) clone, Whitespace storage optimizations

Suggestions:
- Generally, `String`
- If you deal mostly with string literals but want some flexibility (like
  [clap](https://github.com/clap-rs/clap/)), generally you'll want
  `Cow<'static, str`>
- If a profiler says your strings are a problem:
  - Try different crates and settings for that crate out with a profiler
  - O(1) clones are important when doing a lot of clones.  For one-off allocations, they are slower.
  - For short-lived programs, look into string interning

Terms:
- Heap: will store strings in heap-allocated memory
- Inline: will store small-enough strings on the stack

# Results

`new` summary:
[![`new`](runs/2022-03-30/new/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/new/report/index.html)

See [more details](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/new/report/index.html)

`clone` summary:
[![`clone`](runs/2022-03-30/clone/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/clone/report/index.html)

See [more details](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/clone/report/index.html)

`access` summary:
[![`access`](runs/2022-03-30/access/report/lines.svg)](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/access/report/index.html)

See [more details](https://htmlpreview.github.io/?https://github.com/epage/string-benchmarks-rs/blob/master/runs/2022-03-30/access/report/index.html)

# Special Thanks

- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
