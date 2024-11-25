<h1 align="center">iterate-trait</h1>
<div align="center">
  <strong>
    Experiment with methods on IntoIterator
  </strong>
</div>

<br />

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/iterate-trait">
    <img src="https://img.shields.io/crates/v/iterate-trait.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/iterate-trait">
    <img src="https://img.shields.io/crates/d/iterate-trait.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/iterate-trait">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>

<div align="center">
  <h3>
    <a href="https://docs.rs/iterate-trait">
      API Docs
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/iterate-trait/releases">
      Releases
    </a>
    <span> | </span>
    <a href="https://github.com/yoshuawuyts/iterate-trait/blob/master.github/CONTRIBUTING.md">
      Contributing
    </a>
  </h3>
</div>

## Why does this project exist?

This project asks the question: what if we used `IntoIterator` everywhere
instead of `Iterator`? This becomes relevant for generator blocks, which
themselves may contain `!Send` items, but that doesn't mean that the type
returned by `gen {}` needs to be `!Send` too. This crate follows Swift's
example, making it so all operations happen on a base builder type - which
has one final operation that converts it into an actual iterable.

The other reason is that in bounds we already always use `IntoIterator`. For
example the `collect` method takes `A: IntoIterator`, not `A: Iterator`. In
function bounds there is rarely a reason to use `Iterator` directly; typically the
only reason we don't is because it's more effort to type.

## Example of `Iterator`'s limitations

Here's a practical case people are bound to hit when writing generator
blocks, which can't be fixed unless generator returns `IntoIterator`:

```rust
// A gen block that holds some `!Send` type across a yield point
let iter = gen {
    let items = my_data.lock(); // ← `MutexGuard: !Send`
    for item in items {
        yield item;
    }
};

// ## Option 1
let iter = gen { ... };      // 1. Desugars to `impl Iterator + !Send`
thread::spawn(move || {      // 2. ❌ Can't move `!Send` type across threads
    for item in iter { ... }
}).unwrap();

// ## Option 2
let iter = gen { ... };      // 1. Desugars to `impl IntoIterator + Send`
thread::spawn(move || {      // 2. ✅ Move `Send` type across threads
    for item in iter { ... } // 3. Obtain `impl Iterator + !Send`
}).unwrap();
```

## Why did you choose these names?

This crate essentially reframes `IntoIterator` into the main interface for
iteration. However the name `IntoIterator` suggests it is a mere supplement
to some other `Iterator` trait. `Iterator` also has another quirk: it's a
trait that's named after a noun, rather than a verb. Think of `Read`,
`Write`, `Send` - these are all verbs.

The closest prior art for this in the stdlib I could find was the `Hash` /
`Hasher` pair. The main trait `Hash` is the subject of the hashing, with
`Hasher` containing all the hash state. This makes `Hasher` very similar to
`Iterator`, and hints the better name for `IntoIterator` might in fact be `Iterate`.

This just leaves us with what to do about `FromIterator`, which currently
exists as a dual to `IntoIterator`. But interestingly it also exists as a
dual to [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html),
where rather than creating a new container it can be used to extend an
existing collection. This is also used in the unstable [`collect_into`
method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect_into).
It's for this reason that we've renamed `FromIterator` to `Collect`. All
together this changes the names to:

- `IntoIterator` → `Iterate`
- `Iterator` → `Iterator`
- `FromIterator` → `Collect`

## Installation
```sh
$ cargo add iterate-trait
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

[contributing]: https://github.com/yoshuawuyts/iterate-trait/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/iterate-trait/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/iterate-trait/labels/help%20wanted

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
