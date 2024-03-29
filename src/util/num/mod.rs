use crate::util::diag::err::CompilerError;

use super::scan::Scanner;
use super::diag::{Diagnostics, Status, Diagnostic, CompilerDiagnostic};
use super::diag::{ok, error, warning};

#[derive(Debug)]
pub enum NumberType {
    Float(f64),
    Int(i128)
}

impl Default for NumberType {
    fn default() -> Self {
        return NumberType::Int(i128::default());
    }
}

impl NumberType {
    pub fn float(&self) -> Option<&f64> {
        match self {
            Self::Float(flt) => Some(flt),
            _ => None
        }
    }

    pub fn int(&self) -> Option<&i128> {
        match self {
            Self::Int(int) => Some(int),
            _ => None
        }
    }
}

// This is going to be fun
    // TODO: Move this into a NumberParser
    // - [x] simple number literals
    // - [ ] floats
    // - [x] hexadecimal notation
    // - [x] binary notation
    // - [x] octal notation
    // - [ ] scientific notation
    // - [ ] digit separators (1'000'000 -> 1000000)
    // - [ ] number suffixes
    // - [ ] hexadecimal floating points (see https://github.com/libsdl-org/SDL/blob/5b696996cdd94be95ccfe63b8693e0134fb2d571/src/audio/SDL_audiotypecvt.c#L104)
    // See https://stackoverflow.com/questions/4825824/hexadecimal-floating-constant-in-c too
pub struct NumberParser<'num> {
    pub diag: &'num mut Diagnostics,
    pub numtype: NumberType,
    pub scan: Scanner<'num, u8, 3>,
}

impl<'num> NumberParser<'num> {
    pub fn new(source: &'num [u8], diag: &'num mut Diagnostics) -> NumberParser<'num> {
        NumberParser {
            diag,
            numtype: NumberType::default(),
            scan: Scanner::new(source, Some(b'\n')),
        }
    }


    // we use a bool here since there is only 1 type of error that can happen
    // and using Status would make things MUCH harder
    #[inline]
    fn push_int_digit(&mut self, digit: &u8, base: usize) -> bool {
        let conv_digit = match self.convert_digit(digit) {
            Some(d) => d,
            None => return false // error
        };
        self.numtype = NumberType::Int((self.numtype.int().unwrap() * base as i128  + conv_digit as i128).into());
        return true;
    }

    // Supports even more than hexadecimal. Ever wanted base 36? We have it here! Kind of.
    fn convert_digit(&mut self, digit: &u8) -> Option<u8> {
        match digit {
            b'0' ..= b'9' => Some(digit - 48),
            b'A' ..= b'Z' => Some(digit - 55),
            b'a' ..= b'z' => Some(digit - 87),
            _ => None
        }
    }

    // FIXME: refactor this please
    pub fn binary(&mut self) -> Status {
        dbg!(*self.scan.peek(2).unwrap_or(&0) as char);
        match self.scan.nth(1) {
            Some(digit @ (b'0' | b'1')) => if !self.push_int_digit(digit, 2) { return error!(CompilerError::MALFORMED_NUMBER); },
            Some(b' ' | b'\n' | b'\t') | None => return ok!(),
            Some(b'l' | b'L' | b'z' | b'Z' | b'u' | b'U') => return self.suffix(),
            _ => return error!(CompilerError::MALFORMED_NUMBER)
        }
        self.scan.ptr -= 1;
        return self.binary()
    }

    pub fn octal(&mut self) -> Status {
        match self.scan.nth(1) {
            Some(digit @ b'0' ..= b'7') => if !self.push_int_digit(digit, 8) { return error!(CompilerError::MALFORMED_NUMBER); },
            Some(b' ' | b'\n' | b'\t') | None => return ok!(),
            Some(b'l' | b'L' | b'z' | b'Z' | b'u' | b'U') => return self.suffix(),
            _ => return error!(CompilerError::MALFORMED_NUMBER)
        }
        self.scan.ptr -= 1;
        return self.octal()
    }

    pub fn decimal(&mut self) -> Status {
        match self.scan.peek(0) {
            Some(digit @ b'0' ..= b'9') => if !self.push_int_digit(digit, 10) { return error!(CompilerError::MALFORMED_NUMBER); },
            Some(b' ' | b'\n' | b'\t') | None => return ok!(),
            Some(b'l' | b'L' | b'z' | b'Z' | b'u' | b'U') => return self.suffix(),
            _ => return error!(CompilerError::MALFORMED_NUMBER)
        }
        self.scan.next();
        return self.decimal()
    }

    pub fn hex(&mut self) -> Status {
        match self.scan.nth(1) {
            Some(digit @ (b'0' ..= b'9'
                | b'A' ..= b'F'
                | b'a' ..= b'f')) => if !self.push_int_digit(digit, 16) { return error!(CompilerError::MALFORMED_NUMBER); },
            Some(b' ' | b'\n' | b'\t') | None => return ok!(),
            Some(b'l' | b'L' | b'z' | b'Z' | b'u' | b'U') => return self.suffix(),
            _ => return error!(CompilerError::MALFORMED_NUMBER)
        }
        self.scan.ptr -= 1;
        return self.hex()
    }


    pub fn init(&mut self) -> Status {
        match self.scan.peek(0) {
            Some(b'0') => match (self.scan.peek(1), self.scan.peek(2)) {
                // 0xxNUM
                (Some(b'x' | b'X'), Some(b'x' | b'X')) => {
                    self.scan.next(); // advance by one since octal() advances by another automatically
                    return self.octal();
                },

                // Unsure about these but let's see
                // 022NUM
                (Some(b'2'), Some(b'2')) => return self.octal(),
                // 066NUM
                (Some(b'6'), Some(b'6')) => return self.octal(),

                // 0xNUM
                (Some(b'x' | b'X'), _) => return self.hex(),
                // PLACEHOLDER FALSE! Only if a RUNTIME FEATURE (in this case an extension) IS TURNED ON!
                // NO CURRENT SUPPORT OF RUNTIME FEATURES!
                // 0oNUM
                (Some(b'o' | b'O'), _) if false => return self.octal(),
                // 0bNUM
                (Some(b'b' | b'B'), _) => return self.binary(), // same as down here, 0b is an extension

                // 0NUM
                (Some(b'0' ..= b'9'), _) => return self.octal(),
                _ => todo!("unimplemented yet")
            },
            Some(b'0' ..= b'9') => return self.decimal(),
            _ => ok!()
        }
    }

    pub fn suffix(&mut self) -> Status {
        // TODO
        ok!()
    }

    pub fn num(&mut self) {
        match self.init() {
            Some(ref stat) => {
                self.diag.push(Diagnostic { diagnostic: stat.clone(), location: self.scan.location });
                match stat {
                    CompilerDiagnostic::Error(_) => (),
                    CompilerDiagnostic::Warning(_) => return self.num(),
                }
            }
            
            None => ()
        }
    }

    pub fn get_num(&mut self) -> &NumberType {
        &self.numtype
    }
}
