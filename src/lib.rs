//! `stub!()` is a better version of `todo!()`
//!
//! `stub!()` can be assigned to a variable: `let username = stub!(String)`.
//!
//! This allows you to specify just type of the variable and continue working on other code, then come back later and specify its value.
//!
//! ## Examples
//!
//! ```
//! # #[macro_use] extern crate stub_macro;
//! # fn main() {
//! pub fn assign() {
//!     // you can assign stub!() to a variable
//!     let username = stub!(String);
//!     println!("Hello {username}")
//! }
//!
//! pub fn return_position() -> String {
//!     // you can use stub!() like todo!() in return position
//!     stub!()
//! }
//!
//! pub fn infer_type() {
//!     // you can let the compiler automatically infer the type
//!     let status = stub!();
//!     if status { println!("Success") }
//! }
//!
//! pub fn explicit_type() {
//!     // you can specify the type explicitly
//!     let status: bool = stub!();
//!     if status { println!("Success") }
//! }
//!
//! pub fn custom_message() {
//!     // you can add a custom message
//!     let status: bool = stub!("Send a request to GitHub");
//!     if status { println!("Success") }
//! }
//!
//! pub fn impl_example() -> impl Iterator<Item=u32> {
//!     // you can use stub!() in return position even with `impl Trait` return type
//!     // note: `impl Trait` must be written as `impl dyn Trait` due to `macro_rules!` limitation
//!     stub!(impl dyn Iterator<Item=u32>)
//! }
//!
//! pub fn explicit_type_with_message_example() -> u32 {
//!     // you can add
//!     stub!(u32, "Assigned to: {}", "John")
//! }
//!
//! pub fn explicit_type_example() -> u32 {
//!     stub!(u32)
//! }
//!
//! pub fn implicit_type_with_message_example() -> u32 {
//!     stub!("Assigned to: {}", "John")
//! }
//!
//! pub fn implicit_type_example() -> u32 {
//!     stub!()
//! }
//! # }
//! ```
//!
//! ## Behavior
//!
//! When a stub is invoked, it will panic like a `todo!()` macro.
//! However, unlike a `todo!()` macro, it will not make the subsequent parts of your code unreachable.
//!
//! If a custom message is provided, it will be included in the panic message.
//!
//! ## Notes
//!
//! - The stub! macro is intended for use during development and should be
//!   replaced with actual implementations before production use.
//! - When using `impl Trait` in return position, you must use `impl dyn Trait` in the macro invocation due to a limitation in `macro_rules!`
//!

#![no_std]

/// See the crate-level documentation for the overview of this macro
#[macro_export]
macro_rules! stub {
    (impl $ty:ty) => {{
        $crate::_stub::<Box<$ty>>()
    }};
    ($ty:ty, $msg:literal$(, $args:tt)*) => {{
        $crate::_stub_msg::<$ty>(&format!($msg$(, $args)*))
    }};
    ($ty:ty) => {{
        $crate::_stub::<$ty>()
    }};
    ($msg:literal$(, $args:tt)*) => {{
        $crate::_stub_msg(&format!($msg$(, $args)*))
    }};
    () => {{
        $crate::_stub()
    }};
}

#[doc(hidden)]
pub fn _stub<T>() -> T {
    todo!()
}

#[doc(hidden)]
pub fn _stub_msg<T>(msg: &str) -> T {
    todo!("{msg}")
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn test_panic() {
        stub!()
    }
}
