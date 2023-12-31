/// A region of memory containing at least `N` `T`s.
pub struct MinSlice<T, const N: usize> {
    /// The bounded region of memory. Exactly `N` `T`s.
    pub head: [T; N],
    /// Zero or more remaining `T`s after the `N` in the bounded region.
    pub tail: T,
}
impl <T, const N: usize> MinSlice<T, N> {
    /// Construct a `MinSlice` from a slice of at least `N` `T`s.
    pub fn from_slice(slice: &[T])-> MinSlice<T, N> {
        assert!(slice.len() >= N);
        let mut head: [T; N] = slice::from_ref(&slice[0]);
        let mut tail: T = Default::default();
        let (head_slice, tail_slice) = slice.split_at(N);
        head.copy_from_slice(head_slice);
        tail = tail_slice[0];
        MinSlice { head, tail }
    }
}

fn main() {
    let slice: &[u8] = b"Hello, world";
    let reference: Option<&u8> = slice.get(6);
    // We know this value is `Some(b' ')`,
    // but the compiler can't know that.
    assert!(reference.is_some());

    let slice: &[u8] = b"Hello, world";
    // Length check is performed when we construct a MinSlice,
    // and it's known at compile time to be of length 12.
    // If the `unwrap()` succeeds, no more checks are needed
    // throughout the `MinSlice`'s lifetime.
    let minslice = MinSlice::<u8, 12>::from_slice(slice).unwrap();
    let value: u8 = minslice.head[6];
    assert_eq!(value, b' ')
}