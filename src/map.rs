//! Helper types for the `map` operation

use super::{Iterate, Iterator};

/// An iterator which maps items from one type to another
#[derive(Debug)]
pub struct Map<I, F> {
    pub(crate) iter: I,
    f: F,
}

impl<I, F> Map<I, F> {
    fn new(iter: I, f: F) -> Map<I, F> {
        Map { iter, f }
    }
}

/// A type that can be converted into a map iterator.
#[derive(Debug)]
pub struct IntoMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F> IntoMap<I, F> {
    pub(crate) fn new(iter: I, f: F) -> Self {
        Self { iter, f }
    }
}

impl<B, I: Iterator, F> Iterator for Map<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
}

impl<B, I: Iterate, F> Iterate for IntoMap<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    type Iterator = Map<I::Iterator, F>;

    fn iterate(self) -> Self::Iterator {
        Map::new(self.iter.iterate(), self.f)
    }
}
