# lazy tables with stuipd wrapping

[![Travis Build Status]][travis]
[![Latest Version]][crates.io]

[Travis Build Status]: https://travis-ci.org/fiji-flo/lazytable.svg?branch=master
[travis]: https://travis-ci.org/fiji-flo/lazytable
[Latest Version]: https://img.shields.io/crates/v/lazytable.svg
[crates.io]: https://crates.io/crates/lazytable

---

## Getting Started

Add the following dependency to your Cargo manifest…
```toml
[dependencies]
lazytable = "0.1"
```

and look at the docs:
* [lazytable at crates.io](https://crates.io/crate/lazytable)
* [lazytable documentation](https://docs.rs/crate/lazytable)

## Why?

[prettytable](https://github.com/phsym/prettytable-rs) is awesome. But wrapping in a teminal is no fun.

## What can it do?

For now **lazytable** only produces a simple table like this:

Given width of `20`:
```
######################
# da | foobar  | bar #
#    | foobar  |     #
# da | foobar! | bar #
######################
```

Without a width or with [prettytable](https://github.com/phsym/prettytable-rs):
```
######################
# da | foobar foobar #
#| bar               #
# da | foobar! | bar #
######################
```

## TODO

* clean up code
* make it configuarable to some extend
* write proper doc
