#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd,)]
/// # Location
/// The Location struct is a utility struct that represents the
/// location of any object in a source file, using only position
/// and line.
// TODO: add column count..? I am pretty sure it can be calculated,
// however..
pub struct Location {
    pub position: usize,
    pub column: usize,
    pub line: usize,
}


impl Location {
    pub fn new() -> Location {
        return Self::default();
    }
}