//! # Lexer
//! This module contains the source of the lexer, also known as Tokenizer.
//! 
//! ## Supported C version
//! Currently, I am trying to support at least C99.
//! Future plans include, but are not limited to:
//! 
//! - [ ] K&R C (the one with the weird arguments)
//! - [x] ANSI C / C89 / C90 (Work in progress)
//! - [x] C99
//! - [ ] C11
//! - [ ] C17
//! - [ ] C23
//! - [ ] GCC Extensions (Work in progress, at least some of them)
//! - [ ] MSVC Extensions
//! - [ ] Plan9 Extensions
//! 
//! Of course there is absolutely no guarantee that I will finish everything
//! in this list.

mod token;
pub use token::TokenType;
pub use token::Token;
pub use token::Atom;

use std::vec;

use crate::util::diag::*;
use crate::util::scan::*;

use phf::phf_map;

static KEYWORDS: phf::Map<&'static [u8], TokenType> = phf_map! {
    b"int"          => TokenType::INT,
    b"long"         => TokenType::LONG,
    b"float"        => TokenType::FLOAT,
    b"double"       => TokenType::DOUBLE,
    b"void"         => TokenType::VOID,
    b"char"         => TokenType::CHAR,
    b"short"        => TokenType::SHORT,
    b"enum"         => TokenType::ENUM,
    b"struct"       => TokenType::STRUCT,
    b"union"        => TokenType::UNION,
    b"_Bool"        => TokenType::BOOL,
    b"_Complex"     => TokenType::COMPLEX,
    b"_Imaginary"   => TokenType::IMAGINARY,
    b"auto"         => TokenType::AUTO,
    b"extern"       => TokenType::EXTERN,
    b"register"     => TokenType::REGISTER,
    b"static"       => TokenType::STATIC,
    b"const"        => TokenType::CONST,
    b"restrict"     => TokenType::RESTRICT,
    b"volatile"     => TokenType::VOLATILE,
    b"unsigned"     => TokenType::UNSIGNED,
    b"signed"       => TokenType::SIGNED,
    b"if"           => TokenType::IF,
    b"else"         => TokenType::ELSE,
    b"for"          => TokenType::FOR,
    b"while"        => TokenType::WHILE,
    b"break"        => TokenType::BREAK,
    b"continue"     => TokenType::CONTINUE,
    b"do"           => TokenType::DO,
    b"goto"         => TokenType::GOTO,
    b"switch"       => TokenType::SWITCH,
    b"case"         => TokenType::CASE,
    b"default"      => TokenType::DEFAULT,
    b"return"       => TokenType::RETURN,
    b"typedef"      => TokenType::TYPEDEF,
    b"sizeof"       => TokenType::SIZEOF,
    b"asm"          => TokenType::ASM,
    // you can add any keyword here, as long there's a TokenType representing it
    // b'_Atomic'   => TokenType::ATOMIC // for example

    // preprocessor
    b"pragma"       => TokenType::PRAGMA,
    b"include"      => TokenType::INCLUDE,
    b"ifdef"        => TokenType::IFDEF,
    b"define"       => TokenType::DEFINE,
    b"ifndef"       => TokenType::IFNDEF,
    b"elif"         => TokenType::ELIF,
    b"endif"        => TokenType::ENDIF,
    b"line"         => TokenType::LINE,
    b"error"        => TokenType::ERROR,
    b"warning"      => TokenType::WARNING, // C23
    b"undef"        => TokenType::UNDEF,
    b"defined"      => TokenType::DEFINED,
};

/// # Lexer
/// The `Lexer` struct contains most of the lexer
/// implementation.
/// 
/// It consists of a scanner, a mutable reference
/// to a diagnostics and a vector of lexed tokens
/// 
/// It currently accepts only ASCII source code
/// and thus string literals, but UTF-8 support
/// is planned.
pub struct Lexer<'lex> {
    diag: &'lex mut Diagnostics,
    scan: Scanner<'lex, u8, 3>, // TODO: Check if I can lower the lookahead by writing some tests.
    tokens: Vec<Token>,
}

impl<'lex> Lexer<'lex> {
    pub fn new(source: &'lex [u8], diag: &'lex mut Diagnostics) -> Self {
        Self {
            scan: Scanner::new(source, b'\n'),
            tokens: vec![],
            diag
        }
    }

    // &mut so we can later change it (??)
    pub fn get_tokens(&mut self) -> &mut Vec<Token> {
        return &mut self.tokens;
    }

    #[inline]
    fn emit_token(&mut self, tok: TokenType) -> Status {
        self.tokens.push(Token::new(tok, self.scan.location.clone()));
        // push the pointer
        self.scan.next();
        return self.init();
    }

    #[inline]
    fn emit_token_double(&mut self, tok: TokenType) -> Status {
        self.emit_token(tok);
        self.scan.nth(2);
        return self.init();
    }

    fn ignore_line(&mut self) -> Status {
        while self.scan.peek(1) != Some(&b'\n') {
            self.scan.next();
        }
        self.scan.next();
        return self.init();
    }

    // Very inefficient I am using .peek() and other weird tricks everywhere! 
    fn ident_or_keyword(&mut self) -> Status {
        // Clear the buffer BEFORE acting on it
        self.scan.reset_buffer();

        // Start pushing first character into self.buffer.
        // I've done This because an identifier can contain
        // a number, just not at the start of the identifier
        // itself. As such we first match for an identifier
        // without numbers, so only a..=z, A..=Z and _, we
        // push that first character and then we match the
        // actual rest of the identifier, which can indeed
        // contain other stuff like numbers.
        let Some(_) = self.scan.peek(0) else {
            self.tokens.push(Token::new(TokenType::EOF, self.scan.location));
            return ok!();
        };
        self.scan.push_to_buffer();

        // Push the next character into self.buffer until it isn't part of an
        // identifier anymore, so if the next char isn't any of the
        // following (a...z, A...Z, _, 0...9) characters.
        while matches!(self.scan.peek(1), Some(b'a' ..= b'z' | b'A' ..= b'Z' | b'0' ..= b'9' | b'_')) {
            self.scan.push_to_buffer();
            self.scan.next();
        }

        let Some(ref buf) = self.scan.get_from_buffer() else {
            self.tokens.push(Token::new(TokenType::EOF, self.scan.location));
            return ok!();
        };

        // Check if the current identifier is actually a keyword or, well, just an identifier.
        let ident = TokenType::IDENTIFIER(self.scan.buffer.clone().unwrap()); // TODO: Remove this .clone()
        let kw = KEYWORDS.get(buf).unwrap_or(&ident);
        
        self.scan.buffer.as_mut().unwrap();
        return self.emit_token(kw.to_owned());
    }

    // Just adds the number to the buffer without parsing it
    fn number(&mut self) -> Status {
        ok!()   
    }


    fn comment(&mut self) -> Status {
        let mut cur = self.scan.peek(0);
        while cur != Some(&b'\n') {
            // If cur is None, we reached EOF, so we terminate the lexing stage.
            if cur == None {
                return ok!();
            }
            self.scan.next();
            cur = self.scan.peek(0);
        }
        return self.init();
    }

    fn multiline_comment(&mut self) -> Status {
        todo!("TBD");
    }

    fn init(&mut self) -> Status {
        let Some(current) = self.scan.peek(0) else {
            // Push EOF
            self.tokens.push(Token::new(TokenType::EOF, self.scan.location));
            return ok!();
        };

        match (*current, self.scan.peek(1)) {
                // Shebang, we ignore it, maybe we shouldn't
            (b'#', Some(b'!')) => return self.ignore_line(),
            (b'#', Some(b'#')) => return self.emit_token(TokenType::HASHTWICE),
            (b'#', _) => return self.emit_token(TokenType::HASH),
            (b'a' ..= b'z' | b'A' ..= b'Z' | b'_', _) => return self.ident_or_keyword(),
            (b'0' ..= b'9', _) => return self.number(),
            (b'(', _) => return self.emit_token(TokenType::LEFT_PAREN),
            (b'[', _) => return self.emit_token(TokenType::LEFT_BRACKET),
            (b'{', _) => return self.emit_token(TokenType::LEFT_BRACE),
            (b')', _) => return self.emit_token(TokenType::RIGHT_PAREN),
            (b']', _) => return self.emit_token(TokenType::RIGHT_BRACKET),
            (b'}', _) => return self.emit_token(TokenType::RIGHT_BRACE),

            (b';', _) => return self.emit_token(TokenType::SEMICOLON),
            (b':', _) => return self.emit_token(TokenType::COLON),
            (b',', _) => return self.emit_token(TokenType::COMMA),
            (b'?', _) => return self.emit_token(TokenType::QUESTION),

            // TODO: add self.peek_double() and self.emit_token_triple()
            (b'.', Some(b'.')) => match self.scan.peek(2) { // A bit messy I'm sorry
                Some(b'.') => {
                    self.scan.nth(3);
                    return self.emit_token(TokenType::TRIPLET);
                },
                _ => return self.emit_token(TokenType::DOT)
            },

            (b'.', _) =>  return self.emit_token(TokenType::TRIPLET),

            // operators
            (b'+', Some(b'=')) => return self.emit_token_double(TokenType::PLUSEQ),
            (b'+', Some(b'+')) => return self.emit_token_double(TokenType::PLUSPLUS),
            (b'+', _)          => return self.emit_token(TokenType::PLUS),

            (b'-', Some(b'=')) => return self.emit_token_double(TokenType::MINUSEQ),
            (b'-', Some(b'-')) => return self.emit_token_double(TokenType::MINUSMINUS),
            (b'-', Some(b'>')) => return self.emit_token_double(TokenType::ARROW),
            (b'-', _)          => return self.emit_token(TokenType::MINUS),

            (b'*', Some(b'=')) => return self.emit_token_double(TokenType::ASTERISKEQ),
            (b'*', _)          => return self.emit_token(TokenType::ASTERISK),

            (b'/', Some(b'/')) => self.comment(),
            (b'/', Some(b'*')) => self.multiline_comment(),
            (b'/', Some(b'=')) => return self.emit_token_double(TokenType::SLASHEQ),
            (b'/', _)          => return self.emit_token(TokenType::SLASH),

            (b'=', Some(b'=')) => return self.emit_token_double(TokenType::EQ),
            (b'=', _)          => return self.emit_token(TokenType::DOUBLEEQ),

            (b'!', Some(b'=')) => return self.emit_token_double(TokenType::NEQ),
            (b'!', _)          => return self.emit_token(TokenType::BANG),

            (b'~', _)          => return self.emit_token(TokenType::TILDE),

            // TODO: add self.peek_double() and self.emit_token_triple() here too
            (b'<', Some(b'=')) => return self.emit_token_double(TokenType::LESSTHAN),
            (b'<', Some(b'<')) => match self.scan.peek(2) {
                Some(b'=') => {
                    self.scan.nth(3);
                    return self.emit_token(TokenType::DOUBLELESSEQ);
                },
                _ => return self.emit_token_double(TokenType::DOUBLELESS),
            },
            (b'<', _) => return self.emit_token(TokenType::LESS),

            (b'>', Some(b'=')) => return self.emit_token_double(TokenType::GREATERTHAN),
            (b'>', Some(b'>')) => match self.scan.peek(2) {
                Some(b'=') => {
                    self.scan.nth(3);
                    return self.emit_token(TokenType::DOUBLEGREATEREQ);
                },
                _ => return self.emit_token_double(TokenType::DOUBLEGREATER),
            },
            (b'>', _) => return self.emit_token(TokenType::GREATER),

            (b' ' | b'\t' | b'\n' | b'\r', _) => {
                self.scan.next();
                return self.init();
            }

            _ => ok!() // NO UTF-8 Support for now
        }
    }

    pub fn lex(&mut self) {
        match self.init() {
            Some(ref stat) => {
                self.diag.push(Diagnostic { diagnostic: stat.clone(), location: self.scan.location });
                match stat {
                    CompilerDiagnostic::Error(_) => (),
                    CompilerDiagnostic::Warning(_) => return self.lex(), // continue like nothing happened.
                }
            }
            
            None => ()
        }
    }
}
