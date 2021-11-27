mod util;

const ρ0: f64 = 0.5;
const τ0: f64 = 0.75;

const ρd: f64 = 0.25;
const τd: f64 = 1.0;

pub struct PackedMemoryArray<T> {
    size: usize,
    capacity: usize,
    segment_size: u8,
    depth: u8,
    Δρ: f64,
    Δτ: f64,
    data: Vec<T>,
}

impl<T> PackedMemoryArray<T> {
    /// Creates a new PMA.
    pub fn new() -> Self {
        Self {
            size: 0,
            capacity: 1 << 4,
            segment_size: 4,
            depth: 0,
            Δρ: 0.0,
            Δτ: 0.0,
            data: Vec::new(),
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
