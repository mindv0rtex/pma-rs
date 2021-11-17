pub struct PackedMemoryArray<T> {
    data: Vec<T>,
}

impl<T> PackedMemoryArray<T> {
    /// Creates a new PMA.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    /// Inserts a value into the PMA.
    pub fn insert(&mut self, _value: T) {
        unimplemented!()
    }

    /// Deletes a value from the PMA.
    pub fn delete(&mut self, _value: &T) -> bool {
        unimplemented!()
    }

    /// Finds the index of the first element no smaller than the value.
    pub fn search(&self, _value: &T) -> usize {
        unimplemented!()
    }

    /// Returns the number of elements in each leaf in the PMA region defined by start and end indexes.
    fn count(&self, _start: usize, _end: usize) -> impl IntoIterator<Item = usize> {
        [0].into_iter()
    }

    /// Spreads elements from a node evenly among the leaves in the subtree rooted at that node.
    fn rebalance(&mut self, _start: usize, _end: usize) {
        unimplemented!()
    }

    /// Double the PMA size.
    fn grow(&mut self) {
        unimplemented!()
    }

    /// Halve the PMA size.
    fn shrink(&mut self) {
        unimplemented!()
    }
}
