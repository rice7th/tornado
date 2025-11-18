//! # Frontend
//! Tornado's frontend is responsible for lexing (tokenizing), parsing, and pre-
//! processing the source files, turning them into an IR representation.
pub mod lex;
pub mod par;
pub mod cpp;