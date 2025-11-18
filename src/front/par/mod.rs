//! # Parser
//! Tornado's C parser.
// TODO: Add actual grammar definition here

use expr::Literal;

use crate::util::{diag::*, num::NumberParser, scan::Scanner};

use self::expr::{Expr, LiteralType};

use super::lex::{Atom, Token, TokenType};

mod ast;
mod expr;
mod stmt;


// TODO: Support also statements
pub struct Parser<'par> {
    diag: &'par mut Diagnostics,
    scan: Scanner<'par, Token, 3>,
    ast: Option<Expr>
}

impl<'par> Parser<'par> {
    pub fn new(tokens: &'par [Token], diag: &'par mut Diagnostics) -> Parser<'par> {
        Self {
            scan: Scanner::new(tokens, None),
            ast: None,
            diag,
        }
    }

    /***====----------------------------------------------------------------+
     |                                                                      |
     |    BASE GRAMMAR - for testing purposes!                              |
     |    Stolen from crafting interpreters                                 |
     |                                                                      |
     |    expression     -> equality ;                                      |
     |    equality       -> comparison ( ( "!=" | "==" ) comparison )* ;    |
     |    comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;    |
     |    term           -> factor ( ( "-" | "+" ) factor )* ;              |
     |    factor         -> unary ( ( "/" | "*" ) unary )* ;                |
     |    unary          -> ( "!" | "-" ) unary | primary ;                 |
     |    primary        -> NUMBER | STRING | "true" | "false" | "nil"      |
     |                | "(" expression ")" ;                                |
     |                                                                      |
     +----------------------------------------------------------------====***/

    pub fn expression(&mut self) -> Box<Option<Expr>> {
        return self.term();
    }

    pub fn equality(&mut self) -> Box<Option<Expr>> {
        // equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
        let mut expr = self.comparison();
        while matches!(
            self.scan.peek(1),
            Some(Token { tokentype: TokenType::DOUBLEEQ | TokenType::NEQ, .. })
        ) {
            self.scan.next();
            let op = self.scan.peek(0);
            let rhs = self.comparison();
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn comparison(&mut self) -> Box<Option<Expr>> {
        // comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let mut expr = self.term();
        while matches!(
            self.scan.peek(1),
            Some(Token {
                tokentype: TokenType::GREATER | TokenType::GREATEREQ
                         | TokenType::LESS    | TokenType::LESSEQ,
                ..
            })
        ) {
            self.scan.next();
            let op = self.scan.peek(0);
            let rhs = self.term();
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn term(&mut self) -> Box<Option<Expr>> {
        // term           -> factor ( ( "-" | "+" ) factor )* ;
        let mut expr = self.factor(); // Atom(13)
        while matches!(
            dbg!(self.scan.peek(1)),
            Some(Token {
                tokentype: TokenType::MINUS | TokenType::PLUS,
                ..
            })
        ) {
            self.scan.next();
            let op = self.scan.peek(0);
            let rhs = self.factor();
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn factor(&mut self) -> Box<Option<Expr>> {
        // factor         -> unary ( ( "/" | "*" ) unary )* ;  
        let mut expr = self.unary();
        while matches!(
            self.scan.peek(1),
            Some(Token {
                tokentype: TokenType::SLASH | TokenType::ASTERISK,
                ..
            })
        ) {
            self.scan.next();
            let op = self.scan.peek(0);
            let rhs = self.unary();
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn unary(&mut self) -> Box<Option<Expr>> {
        // unary          -> ( "!" | "-" ) unary | primary ;
        if matches!(
            self.scan.peek(0),
            Some(Token {
                tokentype: TokenType::BANG | TokenType::MINUS,
                ..
            })
        ) {
            self.scan.next();
            let op = self.scan.peek(0);
            let rhs = self.unary();
            return Expr::unary(op.cloned(), rhs);
        }
        return self.primary();
    }

    pub fn primary(&mut self) -> Box<Option<Expr>> {
        // primary        -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;    
        // In true C fashion, true, false and null are actually macros, not literals.
        match self.scan.peek(0) {
            Some(Token {
                tokentype: TokenType::ATOM(atom),
                ..
            }) => match atom {
                Atom::STRING(string) => return Box::new(Some(Expr::Value(Literal::new(string.clone(), LiteralType::Str)))),
                Atom::CHAR(chr)      => return Box::new(Some(Expr::Value(Literal::new(chr.clone(), LiteralType::Chr)))),
                Atom::NUM(num)       => return Box::new(Some(Expr::Value(Literal::new(num.clone(), LiteralType::Int)))),
            }
            
            //return Box::new(Some(Expr::Value(atom.clone()))),

            None => todo!(),
            _ => {
                dbg!(self.scan.peek(0));
                todo!()
            }
        }
    }
}