use std::iter::Iterator;

pub struct UnorderedTriples<'a, T> {
    source: &'a [T],
    indices: [usize; 3],
}

impl<'a, T> UnorderedTriples<'a, T> {
    fn current(&self) -> [T; 3]
    where
        T: Copy,
    {
        [
            self.source[self.indices[0]],
            self.source[self.indices[1]],
            self.source[self.indices[2]],
        ]
    }
}

impl<'a, T> Iterator for UnorderedTriples<'a, T>
where
    T: Copy,
{
    type Item = [T; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.indices[2] < self.indices[1] {
            self.indices[2] += 1;
            Some(self.current())
        } else if self.indices[1] < self.indices[0] {
            self.indices[1] += 1;
            self.indices[2] = 0;
            Some(self.current())
        } else if self.indices[0] < self.source.len() - 1 {
            self.indices[0] += 1;
            self.indices[1] = 0;
            self.indices[2] = 0;
            Some(self.current())
        } else {
            None
        }
    }
}

pub struct UnorderedPairs<'a, T> {
    source: &'a [T],
    full_index: usize,
    partial_index: usize,
}

impl<'a, T> UnorderedPairs<'a, T> {
    fn current(&self) -> (T, T)
    where
        T: Copy,
    {
        (
            self.source[self.full_index],
            self.source[self.partial_index],
        )
    }
}

impl<'a, T> Iterator for UnorderedPairs<'a, T>
where
    T: Copy,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<(T, T)> {
        if self.partial_index < self.full_index {
            self.partial_index += 1;
            Some(self.current())
        } else if self.full_index < self.source.len() - 1 {
            self.full_index += 1;
            self.partial_index = 0;
            Some(self.current())
        } else {
            None
        }
    }
}

pub fn iter_unordered_pairs<T>(elems: &[T]) -> UnorderedPairs<T> {
    UnorderedPairs {
        source: elems,
        full_index: 0,
        partial_index: 0,
    }
}

pub fn iter_unordered_triples<T>(elems: &[T]) -> UnorderedTriples<T> {
    UnorderedTriples {
        source: elems,
        indices: [0, 0, 0]
    }
}
