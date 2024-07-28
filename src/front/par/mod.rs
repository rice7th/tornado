//! # Parser
//! Tornado's C parser.
// TODO: Add actual grammar definition here

use expr::Literal;

use crate::util::{diag::*, scan::Scanner};

use self::expr::Expr;

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
        return self.equality();
    }

    pub fn equality(&mut self) -> Box<Option<Expr>> {
        // equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
        let mut expr = self.comparison();
        while matches!(
            self.scan.peek(0),
            Some(Token { tokentype: TokenType::DOUBLEEQ | TokenType::NEQ, .. })
        ) {
            let op = self.scan.peek_back(1);
            let rhs = self.comparison();
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn comparison(&mut self) -> Box<Option<Expr>> {
        // comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
        let mut expr = self.term();
        while matches!(
            self.scan.peek(0),
            Some(Token {
                tokentype: TokenType::GREATER | TokenType::GREATEREQ
                         | TokenType::LESS    | TokenType::LESSEQ,
                ..
            })
        ) {
            let rhs = self.term();
            let op = self.scan.peek_back(1);
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn term(&mut self) -> Box<Option<Expr>> {
        // term           -> factor ( ( "-" | "+" ) factor )* ;
        let mut expr = self.factor();
        while matches!(
            self.scan.peek(0),
            Some(Token {
                tokentype: TokenType::MINUS | TokenType::PLUS,
                ..
            })
        ) {
            // Advance?
            let rhs = self.factor();
            let op = self.scan.peek_back(1);
            expr = Expr::binary(expr.clone(), op.cloned(), rhs);
        }
        return expr;
    }

    pub fn factor(&mut self) -> Box<Option<Expr>> {
        // factor         -> unary ( ( "/" | "*" ) unary )* ;  
        let mut expr = self.unary();
        while matches!(
            self.scan.peek(0),
            Some(Token {
                tokentype: TokenType::SLASH | TokenType::ASTERISK,
                ..
            })
        ) {
            let rhs = self.unary();
            let op = self.scan.peek_back(1);
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
            let rhs = self.unary();
            let op = self.scan.peek_back(1);
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
                Atom::STRING(string) => return Box::new(Some(Expr::Value(Literal::Str(string.as_bytes().to_vec())))),
                Atom::CHAR(chr)      => return Box::new(Some(Expr::Value(Literal::Char(chr.as_bytes()[0])))),
                Atom::NUM(num)       => todo!("Number parsing is almost done™"),
            }
            
            
            //return Box::new(Some(Expr::Value(atom.clone()))),

            None => todo!(),
            _ => todo!()
        }
    }
}