//! Experiment with methods on IntoIterator
//!
//! ## Why does this project exist?
//!
//! This project asks the question: what if we used `IntoIterator` everywhere
//! instead of `Iterator`? This becomes relevant for generator blocks, which
//! themselves may contain `!Send` items, but that doesn't mean that the type
//! returned by `gen {}` needs to be `!Send` too. This crate follows Swift's
//! example, making it so all operations happen on a base builder type - which
//! has one final operation that converts it into an actual iterable.
//!
//! The other reason is that in bounds we already always use `IntoIterator`. For
//! example the `collect` method takes `A: IntoIterator`, not `A: Iterator`. In
//! function bounds there is rarely a reason to use `Iterator` directly; typically the
//! only reason we don't is because it's more effort to type.
//!
//! ## Example of `Iterator`'s limitations
//!
//! Here's a practical case people are bound to hit when writing generator
//! blocks, which can't be fixed unless generator returns `IntoIterator`:
//!
//! ```rust
//! // A gen block that holds some `!Send` type across a yield point
//! let iter = gen {
//!     let items = my_data.lock(); // ← `MutexGuard: !Send`
//!     for item in items {
//!         yield item;
//!     }
//! };
//!
//! // ## Option 1
//! let iter = gen { ... };      // 1. Desugars to `impl Iterator + !Send`
//! thread::spawn(move || {      // 2. ❌ Can't move `!Send` type across threads
//!     for item in iter { ... }
//! }).unwrap();
//!
//! // ## Option 2
//! let iter = gen { ... };      // 1. Desugars to `impl IntoIterator + Send`
//! thread::spawn(move || {      // 2. ✅ Move `Send` type across threads
//!     for item in iter { ... } // 3. Obtain `impl Iterator + !Send`
//! }).unwrap();
//! ```
//!
//! ## Why did you choose these names?
//!
//! This crate essentially reframes `IntoIterator` into the main interface for
//! iteration. However the name `IntoIterator` suggests it is a mere supplement
//! to some other `Iterator` trait. `Iterator` also has another quirk: it's a
//! trait that's named after a noun, rather than a verb. Think of `Read`,
//! `Write`, `Send` - these are all verbs.
//!
//! The closest prior art for this in the stdlib I could find was the `Hash` /
//! `Hasher` pair. The main trait `Hash` is the subject of the hashing, with
//! `Hasher` containing all the hash state. This makes `Hasher` very similar to
//! `Iterator`, and hints the better name for `IntoIterator` might in fact be `Iterate`.
//!
//! This just leaves us with what to do about `FromIterator`, which currently
//! exists as a dual to `IntoIterator`. But interestingly it also exists as a
//! dual to [`Extend`](https://doc.rust-lang.org/std/iter/trait.Extend.html),
//! where rather than creating a new container it can be used to extend an
//! existing collection. This is also used in the unstable [`collect_into`
//! method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect_into).
//! It's for this reason that we've renamed `FromIterator` to `Collect`. All
//! together this changes the names to:
//!
//! - `IntoIterator` → `Iterate`
//! - `Iterator` → `Iterator`
//! - `FromIterator` → `Collect`

#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

pub mod map;

/// A stateful iterator returned by [`Iterate::iterate`].
pub trait Iterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Advances the iterator and returns the next value.
    fn next(&mut self) -> Option<Self::Item>;

    /// How many items do we expect to yield?
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}

/// Provide sequential, iterated access to items.
pub trait Iterate {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type Iterator: Iterator<Item = Self::Item>;

    /// Begin iteration and obtain a stateful [`Iterator`].
    fn iterate(self) -> Self::Iterator;

    /// Maps the values of iter with f.
    fn map<F, B>(self, f: F) -> map::IntoMap<Self, F>
    where
        F: FnOnce(Self::Item) -> B,
        Self: Sized,
    {
        map::IntoMap::new(self, f)
    }

    /// Transforms this iterator into a collection.
    fn collect<B: Collect<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        Collect::collect(self)
    }
}

impl<T> Iterate for T
where
    T: Iterator,
{
    type Item = T::Item;

    type Iterator = T;

    fn iterate(self) -> Self::Iterator {
        self
    }
}

/// Iterate over items and collect them into a value.
pub trait Collect<A>: Sized {
    /// Creates a value from an `Iterate`.
    fn collect<T: Iterate<Item = A>>(iter: T) -> Self;
}
