use std::mem::size_of;

/// Base-2 logarithm, rounded down.
#[inline(always)]
pub const fn floor_lg(x: usize) -> usize {
    const BITS_IN_BYTE: usize = 8;
    size_of::<usize>() * BITS_IN_BYTE - 1 - x.leading_zeros() as usize
}

/// Base-2 logarithm, rounded up.
#[inline(always)]
pub const fn ceil_lg(x: usize) -> usize {
    floor_lg(x - 1) + 1
}

/// Hyperfloor (2^floor_lg(x))
#[inline(always)]
pub const fn hyperfloor(x: usize) -> usize {
    1 << floor_lg(x)
}

/// Hyperceil (2^ceil_lg(x))
#[inline(always)]
pub const fn hyperceil(x: usize) -> usize {
    1 << ceil_lg(x)
}

#[test]
fn test1() {
    assert_eq!(floor_lg(64), 6);
    assert_eq!(ceil_lg(64), 6);
    assert_eq!(ceil_lg(65), 7);
}
