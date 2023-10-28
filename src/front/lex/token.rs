use crate::util::scan::location::Location;
use crate::util::scan::buffer::Buffer;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Token {
    tokentype: TokenType,
    location: Location
}

impl Token {
    pub fn new(tokentype: TokenType, location: Location) -> Token {
        return Token { tokentype, location }
    }
}


#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum TokenType {
    // Datatypes
    INT, LONG, FLOAT, DOUBLE, VOID, CHAR, SHORT,
    ENUM, STRUCT, UNION, BOOL, COMPLEX, IMAGINARY,
    
    // Storage-class specifiers
    AUTO, EXTERN, REGISTER, STATIC,

    // Type qualifiers
    CONST, RESTRICT, VOLATILE,

    // Type Modifiers
    UNSIGNED, SIGNED,
    
    // Control flow
    IF, ELSE, FOR, WHILE, BREAK, CONTINUE, DO, GOTO,
    SWITCH, CASE, DEFAULT, RETURN,

    // Misc
    TYPEDEF, SIZEOF, ASM, // Inline assembler

    // Punctuation, operators and symbols
//  +     -      *         /      !     %
    PLUS, MINUS, ASTERISK, SLASH, BANG, PERCENT,
//  ;          :      |    &          ,      .
    SEMICOLON, COLON, BAR, AMPERSAND, COMMA, DOT,
//  ->     ++        --          =   ...
    ARROW, PLUSPLUS, MINUSMINUS, EQ, TRIPLET,
//  ~      &&               ||         ==
    TILDE, DOUBLEAMPERSAND, DOUBLEBAR, DOUBLEEQ,
//  !=   >        >=           <     <=
    NEQ, GREATER, GREATERTHAN, LESS, LESSTHAN,
//  >>             <<          ^      ?
    DOUBLEGREATER, DOUBLELESS, CARET, QUESTION,
//  #     ##         +=      -=       *=
    HASH, HASHTWICE, PLUSEQ, MINUSEQ, ASTERISKEQ,
//  /=       %=         &=           |=     ^=
    SLASHEQ, PERCENTEQ, AMPERSANDEQ, BAREQ, CARETEQ,
//  >>=              <<=           (           )
    DOUBLEGREATEREQ, DOUBLELESSEQ, LEFT_PAREN, RIGHT_PAREN,
//  [             ]              {           }
    LEFT_BRACKET, RIGHT_BRACKET, LEFT_BRACE, RIGHT_BRACE,

    // Preprocessor (warning -> C23)
    INCLUDE, PRAGMA, IFDEF, DEFINE, IFNDEF, ELIF, ENDIF,
    LINE, ERROR, WARNING, UNDEF, DEFINED,

    // Atoms and Identifiers
    IDENTIFIER(Buffer),
    
    ATOM(Atom),

    // End of File
    EOF
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Atom {
    STRING(Buffer),
    CHAR(Buffer),
    INT(Buffer),
    UINT(Buffer),
    FLOAT(Buffer),
}