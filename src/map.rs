//! Helper types for the `map` operation

use super::{IntoIterator, Iterator};

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

impl<B, I: IntoIterator, F> IntoIterator for IntoMap<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    type IntoIterator = Map<I::IntoIterator, F>;

    fn into_iter(self) -> Self::IntoIterator {
        Map::new(self.iter.into_iter(), self.f)
    }
}
