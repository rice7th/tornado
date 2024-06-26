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
/// A newline of type `T` can be provided in order
/// to tack the line count in the `location` field.
/// 
/// ## Example
/// ```ignore
/// let col = b"My tiny collection";
/// // Lookahead of 4, so the method `self.peek(n)` can access the next
/// // four items, ranging from n = 0 to n = 3.
/// let mut scan: Scanner<u8, 4> = Scanner::new(col, b'\n');
/// assert!(scan.peek(0) == Some(&b'M')); // current item
/// assert!(scan.peek(1) == Some(&b'y')); // next item
/// assert!(scan.peek(2) == Some(&b' ')); // 2nd next item
/// assert!(scan.peek(3) == Some(&b't')); // 3rd next item
/// assert!(scan.peek(4) == None);       // End of the lookahead
/// ```
/// 
/// Since `Scanner` implements `Iterator`, the former can be advanced
/// with `self.next()`:
/// ```ignore
/// assert!(scan.next() == Some(&b'y'));
/// assert!(scan.peek(1) == Some(&b' ')); // next item
/// ```
/// In case the item collection reaches EOF, the lookahead will just
/// return `None`:
/// ```ignore
/// assert!(scan.nth(15) == Some(&b'n'));
/// assert!(scan.peek(0) == Some(&b'n'));
/// assert!(scan.peek(1) == None);
/// ```
pub struct Scanner<'scan, T: PartialEq, const L: usize> {
    pub item_collection: &'scan [T],
    pub ptr: usize,
    pub lookahead: [Option<&'scan T>; L],
    /// ## Lookback
    /// Same as the lookahead, except it holds
    /// previous values. Also `L` size just like
    /// the `lookahead`.
    pub lookback:  [Option<&'scan T>; L],
    /// ## Newline
    /// The Newline is the item `T` that when
    /// encountered it raises the `Location` line
    /// count and resets the column count.
    pub newline: Option<T>,
    pub location: Location,
    pub buffer: Option<Buffer>,
}

impl<'scan, T: PartialEq, const L: usize> Scanner<'scan, T, L> {
    pub fn new(item_collection: &'scan [T], newline: Option<T>) -> Scanner<'scan, T, L> {
        return Scanner {
            item_collection,
            ptr: 0,
            lookahead: array::from_fn(|i| item_collection.get(i)),
            lookback: array::from_fn(|i| item_collection.get(i)),
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

    #[inline]
    pub fn peek_back(&self, n: usize) -> Option<&'scan T> {
        // same TODO applies here
        return self.lookback.get(n).copied().flatten();
    }

    pub fn increment_location(&mut self) {
        self.location.position += 1;
        self.location.column += 1;
        if self.newline != None && self.peek(0) == self.newline.as_ref() {
            self.location.line += 1;
            self.location.column = 0;
        }
    }

    pub fn push_to_buffer(&mut self) {
        if let Some(ref mut buf) = self.buffer {
            buf.size += 1;
        } else {
            // Initialize buffer if no buffer is present
            self.reset_buffer();
            self.buffer.as_mut().unwrap().size += 1;
        }
    }

    pub fn get_from_buffer(&mut self) -> Option<&'scan [T]> {
        let Some(ref buf) = self.buffer else { return None; };
        return self.item_collection.get(buf.start..buf.start + buf.size);
    }

    pub fn reset_buffer(&mut self) {
        let mut buf = Buffer::default();
        buf.start = self.location.position;
        self.buffer = Some(buf);
    }
}

impl<'scan, T: PartialEq,  const L: usize> Iterator for Scanner<'scan, T, L> {
    type Item = &'scan T;

    fn next(&mut self) -> Option<Self::Item> {
        self.ptr += 1;
        self.location.position = self.ptr - 1;
        let next_item = self.item_collection.get(self.ptr - 1);
        self.lookahead = array::from_fn(|i| self.item_collection.get(self.ptr - 1 + i));
        self.lookback  = array::from_fn(|i| self.item_collection.get(self.ptr.saturating_sub(i).saturating_sub(1)));
        match next_item {
            Some(nl) if Some(nl) == self.newline.as_ref() => self.location.line += 1,
            _ => {}
        }
        return next_item;
    }
}