//! # Statements
//! This file contains all the statements supported by C.

#[derive(Clone, Debug)]
// program -> item
// item -> fdec
//    | constdecl
//    | structdecl
//    | enumdecl
//    | uniondecl
//    | typedef
// block -> stmt ';' | '{' (stmt;)* '}'
// stmt -> 
//        | if '(' EXPR ')' block [else block]
//        | for '(' vdecl ';' EXPR ';' EXPR ')' block
//        | while '(' EXPR ')' block
//        | do block while '(' EXPR ')'
//        | vdecl
//        | fdecl
//        | enumdecl
//        | structdecl
//        | uniondecl
//        | return EXPR;
// structdecl -> struct name struct_block
// enumdecl -> enum name enum_block
// uniondecl -> union name union_block
// typedef -> typedef type IDENT;
// item  -> fdecl
// fdecl -> [inline] [static] [volatile] type IDENT(args) block
// type -> [(unsigned | signed)] ['('] (int | char | ptr | ) [')']
// ptr -> *type
// args -> (type IDENT,)*
pub enum Stmt {}