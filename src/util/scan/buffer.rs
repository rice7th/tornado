// FIXME: Move all the buffer related methods from the scanner into the buffer
// itself.

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
/// # Buffer
/// A data structure that holds a pointer and a size
/// to a collection like a slice.
/// Used in the `Scanner` struct to hold keywords and
/// it could be used in the future in other places too.
pub struct Buffer {
    pub start: usize,
    pub size: usize,
}