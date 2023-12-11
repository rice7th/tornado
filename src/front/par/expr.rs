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

use crate::front::lex::{Token, Atom};


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
    Group(Box<Expr>),

    // misc -> ternary | cast
    Misc(MiscExpr),
}

pub enum BinExpr {
//  expr =                            expr[expr]
    Assign(Box<Expr>, Box<Expr>),     Index(Box<Expr>, Box<Expr>),
//  expr . expr                       expr -> expr
    MembOf(Box<Expr>, Box<Expr>),     MembOfRef(Box<Expr>, Box<Expr>),
//  expr + expr                       expr += expr
    Add(Box<Expr>, Box<Expr>),        AddAssign(Box<Expr>, Box<Expr>),
//  expr - expr                       expr -= expr
    Sub(Box<Expr>, Box<Expr>),        SubAssign(Box<Expr>, Box<Expr>),
//  expr * expr                       expr *= expr
    Mul(Box<Expr>, Box<Expr>),        MulAssign(Box<Expr>, Box<Expr>),
//  expr / expr                       expr /= expr
    Div(Box<Expr>, Box<Expr>),        DivAssign(Box<Expr>, Box<Expr>),
//  expr % expr                       expr %= expr
    Mod(Box<Expr>, Box<Expr>),        ModAssign(Box<Expr>, Box<Expr>),
//  expr >> expr                      expr >>= expr
    Shr(Box<Expr>, Box<Expr>),        ShrAssign(Box<Expr>, Box<Expr>),
//  expr << expr                      expr <<= expr
    Shl(Box<Expr>, Box<Expr>),        ShlAssign(Box<Expr>, Box<Expr>),

//  expr | expr                       expr |= expr 
    Or(Box<Expr>, Box<Expr>),         OrAssign(Box<Expr>, Box<Expr>),
//  expr & expr                       expr &= expr
    And(Box<Expr>, Box<Expr>),        AndAssign(Box<Expr>, Box<Expr>),
//  expr ^ expr                       expr ^= expr
    Xor(Box<Expr>, Box<Expr>),        XorAssign(Box<Expr>, Box<Expr>),

//  expr || expr                      expr && expr
    ShOr(Box<Expr>, Box<Expr>),       ShAnd(Box<Expr>, Box<Expr>),

//  expr == expr                      expr != expr
    Eq(Box<Expr>, Box<Expr>),         Neq(Box<Expr>, Box<Expr>),
//  expr < expr                       expr > expr 
    Less(Box<Expr>, Box<Expr>),       Greater(Box<Expr>, Box<Expr>),
//  expr <= expr                      expr >= expr 
    LessAssign(Box<Expr>, Box<Expr>), GreaterAssign(Box<Expr>, Box<Expr>),
}

pub enum UnaryExpr {
//  !expr           ~expr
    Not(Box<Expr>), BinNot(Box<Expr>),
//  ++expr             --expr
    PreInc(Box<Expr>), PreDec(Box<Expr>),
//  expr++              expr--
    PostInc(Box<Expr>), PostDec(Box<Expr>),
//  -
    Neg(Box<Expr>),
//  &expr           *expr
    Ref(Box<Expr>), Deref(Box<Expr>),
//  sizeof(expr)       _Alignof(expr)
    Sizeof(Box<Expr>), Alignof(Box<Expr>),
}

pub enum MiscExpr {
//  (expr) ? expr : expr
    Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
//  (type)expr
    Cast(Box<Expr>), // ???
}