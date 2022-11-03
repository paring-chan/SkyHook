#[macro_export]
/// You can use `break` statement to escape given scope.
/// 
/// # Examples
/// ```rs
/// breakable! ({
///     let a = 10;
///     if a == 11 {
///         break; // does not escape the scope
///     }
/// 
///     a = 11;
///     if a == 11 {
///         break; // escapes the scope
///     }
/// });
/// ```
macro_rules! breakable {
    ($xs:block) => {
        loop { break $xs }
    };
}

#[macro_export]
/// You can use `break` statement to escape given `unsafe` scope.
/// 
/// # Examples
/// ```rs
/// breakable_unsafe! ({
///     let a = 10;
///     if a == 11 {
///         break; // does not escape the scope
///     }
/// 
///     a = 11;
///     if a == 11 {
///         break; // escapes the scope
///     }
/// });
/// ```
macro_rules! breakable_unsafe {
    ($xs:block) => {
        unsafe { loop { break $xs } }
    };
}