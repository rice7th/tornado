#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum CompilerWarning {
    // clang -Wbuiltin-macro-redefined
    /// # UNDEFINED_BUILTIN_MACRO
    /// This warning is emitted when a builtin
    /// macro is undefined, either via the
    /// `#undef` preprocessor directive or via
    /// the `-U` commandline attribute.
    /// 
    /// ## Examples
    /// 
    /// Ths code snippet fails to compile
    /// since `__LINE__` is undefined and
    /// not a known macro or identifier
    /// 
    /// ```c
    /// #undef __LINE__ // Macro undefined here
    /// int main() {
    ///     return __LINE__;
    /// }
    /// ```
    UNDEFINED_BUILTIN_MACRO,

    /// # MISSING_PREPROCESSOR_DIRECTIVE
    /// 
    /// > _Also known as a NULL directive_
    /// 
    /// This error is triggered when a preprocessor
    /// directive is defined via an `#` but no actual
    /// directive is provided 
    /// 
    /// ## Example
    /// ```c
    /// #include "mylib.h" // OK
    /// # // Missing directive (for example, 'include' or 'define')
    /// int main() {
    ///     return 0;
    /// }
    /// ``` 
    MISSING_PREPROCESSOR_DIRECTIVE,
}