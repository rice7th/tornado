//! # Expressions
//! This file contains all the expressions supported by C.
//! 
//! Currently supported expressions:
//! - [x] Basic arithmetic (+, -, *, /, % and all assignments)
//! - [x] Bitwise (>>, <<, &, |, ~, ^ and all assignments)
//! - [x] Shorthand logic (&&, ||, !)
//! - [x] Comparison (<, >, ==, !=, <=, >=)
//! - [x] Memory operations (&, *, sizeof, _Alignof)
//! - [x] Structs (., ->)
//! - [ ] Type operations (typeof, cast)
//! - [x] Ternary (? :)

use crate::front::lex::{Token, TokenType};


#[derive(Clone, Debug)]
// expr -> binary | unary | atom | group
// op ->  "+" | "-" | "*" | "/" | "!" | "%"
    //  | ":" | "|" | "&" | "." | "->" | "++" 
    //  | "--" | "=" | "..." | "~" | "&&" 
    //  | "||" | "==" | "!=" | ">" | ">=" 
    //  | "<" | "<=" | ">>" | "<<" | "^" 
    //  | "?" | "+=" |"-=" | "*=" | "/=" 
    //  | "%=" | "&=" | "|=" | "^=" | ">>="
    //  | "<<=" | "sizeof" | "_Alignof"
pub enum Expr {
    // binary -> expr op expr
    Binary(BinExpr),

    // unary -> op expr
    Unary(UnaryExpr),

    // atom -> NUMBER | STRING | CHAR
    Atom(Atom),
    
    // group -> "(" expr ")"
    Group(Box<Option<Expr>>),

    // misc -> ternary | cast
    Misc(MiscExpr),
}

#[derive(Clone, Debug)]
pub enum BinExpr {
//  ident = expr                      expr[expr]
    Assign(Box<Option<Expr>>, Box<Option<Expr>>),     Index(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr . expr                       expr -> expr
    MembOf(Box<Option<Expr>>, Box<Option<Expr>>),     MembOfRef(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr + expr                       expr += expr
    Add(Box<Option<Expr>>, Box<Option<Expr>>),        AddAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr - expr                       expr -= expr
    Sub(Box<Option<Expr>>, Box<Option<Expr>>),        SubAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr * expr                       expr *= expr
    Mul(Box<Option<Expr>>, Box<Option<Expr>>),        MulAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr / expr                       expr /= expr
    Div(Box<Option<Expr>>, Box<Option<Expr>>),        DivAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr % expr                       expr %= expr
    Mod(Box<Option<Expr>>, Box<Option<Expr>>),        ModAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr >> expr                      expr >>= expr
    Shr(Box<Option<Expr>>, Box<Option<Expr>>),        ShrAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr << expr                      expr <<= expr
    Shl(Box<Option<Expr>>, Box<Option<Expr>>),        ShlAssign(Box<Option<Expr>>, Box<Option<Expr>>),

//  expr | expr                       expr |= expr 
    Or(Box<Option<Expr>>, Box<Option<Expr>>),         OrAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr & expr                       expr &= expr
    And(Box<Option<Expr>>, Box<Option<Expr>>),        AndAssign(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr ^ expr                       expr ^= expr
    Xor(Box<Option<Expr>>, Box<Option<Expr>>),        XorAssign(Box<Option<Expr>>, Box<Option<Expr>>),

//  expr || expr                      expr && expr
    ShOr(Box<Option<Expr>>, Box<Option<Expr>>),       ShAnd(Box<Option<Expr>>, Box<Option<Expr>>),

//  expr == expr                      expr != expr
    Eq(Box<Option<Expr>>, Box<Option<Expr>>),         Neq(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr < expr                       expr > expr 
    Less(Box<Option<Expr>>, Box<Option<Expr>>),       Greater(Box<Option<Expr>>, Box<Option<Expr>>),
//  expr <= expr                      expr >= expr 
    LessEq(Box<Option<Expr>>, Box<Option<Expr>>),     GreaterEq(Box<Option<Expr>>, Box<Option<Expr>>),
}

#[derive(Clone, Debug)]
pub enum UnaryExpr {
//  !expr           ~expr
    Not(Box<Option<Expr>>), BinNot(Box<Option<Expr>>),
//  ++expr             --expr
    PreInc(Box<Option<Expr>>), PreDec(Box<Option<Expr>>),
//  expr++              expr--
    PostInc(Box<Option<Expr>>), PostDec(Box<Option<Expr>>),
//  -
    Neg(Box<Option<Expr>>),
//  &expr           *expr
    Ref(Box<Option<Expr>>), Deref(Box<Option<Expr>>),
//  sizeof(expr)       _Alignof(expr)
    Sizeof(Box<Option<Expr>>), Alignof(Box<Option<Expr>>),
}

#[derive(Clone, Debug)]
pub enum MiscExpr {
//  (expr) ? expr : expr
    Ternary(Box<Option<Expr>>, Box<Option<Expr>>, Box<Option<Expr>>),
//  (type)expr
    Cast(Box<Option<Expr>>), // ???
}

macro_rules! expr_bin {
    ($lhs:expr, $rhs:expr, $op:ident) => {
        Box::new(Some(Expr::Binary(BinExpr::$op($lhs, $rhs))))
    };
}

macro_rules! expr_un {
    ($rhs:expr, $op:ident) => {
        Box::new(Some(Expr::Unary(UnaryExpr::$op($rhs))))
    };
}

impl Expr {
    pub fn binary(lhs: Box<Option<Expr>>, op: Option<Token>, rhs: Box<Option<Expr>>) -> Box<Option<Expr>> {
        match op { // Not all binary operations are included. Index for example is a little bit more complex.
            Some(Token { tokentype, .. }) => match tokentype {
                TokenType::EQ              => expr_bin!(lhs, rhs, Assign),      // =
                TokenType::DOT             => expr_bin!(lhs, rhs, MembOf),      // .
                TokenType::ARROW           => expr_bin!(lhs, rhs, MembOfRef),   // ->
                TokenType::PLUS            => expr_bin!(lhs, rhs, Add),         // +
                TokenType::PLUSEQ          => expr_bin!(lhs, rhs, AddAssign),   // +=
                TokenType::MINUS           => expr_bin!(lhs, rhs, Sub),         // -
                TokenType::MINUSEQ         => expr_bin!(lhs, rhs, SubAssign),   // -=
                TokenType::ASTERISK        => expr_bin!(lhs, rhs, Mul),         // *
                TokenType::ASTERISKEQ      => expr_bin!(lhs, rhs, MulAssign),   // *=
                TokenType::SLASH           => expr_bin!(lhs, rhs, Div),         // /
                TokenType::SLASHEQ         => expr_bin!(lhs, rhs, DivAssign),   // /=
                TokenType::PERCENT         => expr_bin!(lhs, rhs, Mod),         // %
                TokenType::PERCENTEQ       => expr_bin!(lhs, rhs, ModAssign),   // %=
                TokenType::DOUBLEGREATER   => expr_bin!(lhs, rhs, Shr),         // >>
                TokenType::DOUBLEGREATEREQ => expr_bin!(lhs, rhs, ShrAssign),   // >>=
                TokenType::DOUBLELESS      => expr_bin!(lhs, rhs, Shl),         // <<
                TokenType::DOUBLELESSEQ    => expr_bin!(lhs, rhs, ShlAssign),   // <<=
                TokenType::BAR             => expr_bin!(lhs, rhs, Or),          // |
                TokenType::BAREQ           => expr_bin!(lhs, rhs, OrAssign),    // |=
                TokenType::AMPERSAND       => expr_bin!(lhs, rhs, And),         // &
                TokenType::AMPERSANDEQ     => expr_bin!(lhs, rhs, AndAssign),   // &=
                TokenType::CARET           => expr_bin!(lhs, rhs, Xor),         // ^
                TokenType::CARETEQ         => expr_bin!(lhs, rhs, XorAssign),   // ^=
                TokenType::DOUBLEBAR       => expr_bin!(lhs, rhs, ShOr),        // ||
                TokenType::DOUBLEAMPERSAND => expr_bin!(lhs, rhs, ShAnd),       // &&
                TokenType::DOUBLEEQ        => expr_bin!(lhs, rhs, Eq),          // ==
                TokenType::NEQ             => expr_bin!(lhs, rhs, Neq),         // !=
                TokenType::LESS            => expr_bin!(lhs, rhs, Less),        // <
                TokenType::LESSEQ          => expr_bin!(lhs, rhs, LessEq),      // <=
                TokenType::GREATER         => expr_bin!(lhs, rhs, Greater),     // >
                TokenType::GREATEREQ       => expr_bin!(lhs, rhs, GreaterEq),   // >=
                _ => Box::new(None)
            },
            _ => Box::new(None)
        }
    }
    pub fn unary(op: Option<Token>, rhs: Box<Option<Expr>>) -> Box<Option<Expr>> {
        match op { // No idea how to differenciate ++expr and expr++, so I am not implementing those for now
            Some(Token { tokentype, .. }) => match tokentype {
                TokenType::BANG       => expr_un!(rhs, Not),    // !
                TokenType::TILDE      => expr_un!(rhs, BinNot), // ~
                TokenType::PLUSPLUS   => expr_un!(rhs, PreInc), // ++
                TokenType::MINUSMINUS => expr_un!(rhs, PreDec), // --
                TokenType::MINUS      => expr_un!(rhs, Neg),    // -
                TokenType::AMPERSAND  => expr_un!(rhs, Ref),    // &
                TokenType::ASTERISK   => expr_un!(rhs, Deref),  // *
                TokenType::SIZEOF     => expr_un!(rhs, Sizeof), // sizeof()
                _ => Box::new(None)
            },
            _ => Box::new(None)
        }
    }
}