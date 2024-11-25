//! Experiment with methods on IntoIterator
//!
//! This project asks the question: what if we used `IntoIterator` everywhere
//! instead of `Iterator`? This becomes relevant for generator blocks, which
//! themselves may contain `!Send` items, but that doesn't mean that the type
//! returned by `gen {}` needs to be `!Send` too.
//!
//! This crate follows Swift's example, making it so all operations happen on a
//! base builder type - which has one final operation that converts it into an
//! actual iterable.

#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, future_incompatible, unreachable_pub)]

pub mod map;

/// A trait for dealing with iterators.
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

/// Conversion into an Iterator.
pub trait IntoIterator {
    /// The type of the elements being iterated over.
    type Item;

    /// Which kind of iterator are we turning this into?
    type IntoIterator: Iterator<Item = Self::Item>;

    /// Creates an iterator from a value.
    fn into_iter(self) -> Self::IntoIterator;

    /// Maps the values of iter with f.
    fn map<F, B>(self, f: F) -> map::IntoMap<Self, F>
    where
        F: FnOnce(Self::Item) -> B,
        Self: Sized,
    {
        map::IntoMap::new(self, f)
    }

    /// Transforms this iterator into a collection.
    fn collect<B: FromIterator<Self::Item>>(self) -> B
    where
        Self: Sized,
    {
        FromIterator::from_iter(self)
    }
}

/// Conversion from an [`IntoIterator`].
pub trait FromIterator<A>: Sized {
    /// Creates a value from an `IntoIterator`.
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self;
}
