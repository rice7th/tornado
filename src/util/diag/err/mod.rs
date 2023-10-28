#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CompilerError {
    /// # MALFORMED_NUMBER
    /// This error is triggered when there is an
    /// error lexing a number.
    /// Usually this error happens when a number
    /// is wrongly defined, for example, or when
    /// 
    /// ## Example
    /// ```c
    /// int main(void) {
    ///     return 0xABCZ; // Malfomed number!
    /// }
    /// ``` 
    /// In this example, `0xABCZ` is a malformed
    /// number since `Z` is not a valid hexadecimal
    /// digit.
    MALFORMED_NUMBER,
    
    /// # UNEXPECTED_EOF
    /// This error is triggered when EOF (End Of File)
    /// is encountered while parsing an expression or
    /// a statement.
    /// 
    /// ## Example
    /// ```c
    /// int main(void) {
    ///     return 0;
    /// // end of file here (Missing '}')
    /// ``` 
    /// In this example, EOF is unexpected since we're
    /// still parsing the body of the `main()` function.
    UNEXPECTED_EOF,

    /// # UNKNOWN_PREPROCESSOR_DIRECTIVE
    /// This error is triggered when invoking a
    /// non-existing preprocessor directive. 
    /// 
    /// It must be one of the following:
    /// `include`, `define`, `pragma`, `undef`, 
    /// `error`, `warning`, `line`, `if`, 
    /// `elif`, `else`, `endif`, `ifdef`, 
    /// `ifndef`.
    ///  
    /// ## Example
    /// ```c
    /// #include "mylib" // OK
    /// #hello // Unknown preprocessor directive
    /// int main() {
    ///     return 0;
    /// }
    /// ``` 
    UNKNOWN_PREPROCESSOR_DIRECTIVE,
}