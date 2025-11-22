//! # Frontend
//! Tornado's frontend is responsible for lexing (tokenizing), parsing, and pre-
//! processing the source files, turning them into an IR representation, which
//! is later compiled down to source code.
//! 
//! Tornado currently targets basic C99, however the plan is to support up to
//! C23 with Gnu extensions.
pub mod lex;
pub mod par;
pub mod cpp;
pub mod sema;