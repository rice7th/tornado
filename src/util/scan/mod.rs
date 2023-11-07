use std::cmp::PartialEq;
use std::array;

pub mod location;
pub mod buffer;

use location::*;
use buffer::*;

#[derive(Debug)]
/// # Scanner
/// A `Scanner` is an object that iterates over an
/// item collection of type `T` with `L` lookahead.
/// 
/// Useful for many iterator based tasks that need a
/// configurable lookahead, like Lexing, Parsing and
/// so on.
/// 
/// A newline of type `T` needs to be provided in
/// order to tack the line count in the `location`
/// field.
/// 
/// ## Example
/// ```rust
/// let col = b"My tiny collection";
/// // Lookahead of 4, so the method `self.peek(n)` can access the next
/// // four items, ranging from n = 0 to n = 3.
/// let mut scan: Scanner<u8, 4> = Scanner::new(col, 10u8);
/// assert!(scan.peek(0) == Some(&b'M')); // current item
/// assert!(scan.peek(1) == Some(&b'y')); // next item
/// assert!(scan.peek(2) == Some(&b' ')); // 2nd next item
/// assert!(scan.peek(3) == Some(&b't')); // 3rd next item
/// assert!(scan.peek(4) == None);       // End of the lookahead
/// ```
/// 
/// Since `Scanner` implements `Iterator`, the former can be advanced
/// with `self.next()`:
/// ```rust
/// assert!(scan.next() == Some(&b'y'));
/// assert!(scan.peek(1) == Some(&b' ')); // next item
/// ```
/// In case the item collection reaches EOF, the lookahead will just
/// return `None`:
/// ```rust
/// assert!(scan.nth(15) == Some(&b'n'));
/// assert!(scan.peek(0) == Some(&b'n'));
/// assert!(scan.peek(1) == None);
/// ```
pub struct Scanner<'scan, T: PartialEq, const L: usize> {
    pub item_collection: &'scan [T],
    pub ptr: usize,
    pub lookahead: [Option<&'scan T>; L],
    /// ## Newline
    /// The Newline is the item `T` that when
    /// encountered it raises the `Location` line
    /// count and resets the column count.
    pub newline: T,
    pub location: Location,
    pub buffer: Option<Buffer>,
}

impl<'scan, T: PartialEq, const L: usize> Scanner<'scan, T, L> {
    pub fn new(item_collection: &'scan [T], newline: T) -> Scanner<'scan, T, L> {
        return Scanner {
            item_collection,
            ptr: 0,
            lookahead: array::from_fn(|i| item_collection.get(i)),
            newline,
            location: Location::default(),
            buffer: None,
        }
    }

    #[inline]
    pub fn peek(&self, n: usize) -> Option<&'scan T> {
        // TODO: Fix this garbage
        return self.lookahead.get(n).copied().flatten();
    }

    pub fn increment_location(&mut self) {
        self.location.position += 1;
        self.location.column += 1;
        if self.peek(0) == Some(&self.newline) {
            self.location.line += 1;
            self.location.column = 0;
        }
    }

    pub fn push_to_buffer(&mut self) {
        // ref mut is a thing what??
        if let Some(ref mut buf) = self.buffer {
            buf.size += 1;
        } else {
            let mut newbuf = Buffer::default();
            newbuf.start = self.location.position;
            self.buffer = Some(newbuf);
        }
    }

    pub fn get_from_buffer(&mut self) -> Option<&'scan [T]> {
        let Some(ref buf) = self.buffer else { return None; };
        return self.item_collection.get(buf.start..=buf.start + buf.size);
    }
}

impl<'scan, T: PartialEq,  const L: usize> Iterator for Scanner<'scan, T, L> {
    type Item = &'scan T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        self.location.position = self.ptr;
        let next_item = self.item_collection.get(self.ptr);
        self.lookahead = array::from_fn(|i| self.item_collection.get(self.ptr + i));
        match next_item {
            Some(nl) if nl == &self.newline => self.location.line += 1,
            _ => {}
        }
        return next_item;
    }
}