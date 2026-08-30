//! `stub!()` can be assigned to a variable:
//!
//! ```rust
//! # #[macro_use] extern crate stub_macro;
//! # fn main() {
//! # fn assign() {
//! let username = stub!(String);
//! # }
//! # }
//! ```
//!
//! This allows you to specify just type of the variable and continue working on other code, then come back later and specify its value.
//!
//! ## Examples
//!
//! ```
//! # #[macro_use] extern crate stub_macro;
//! # #[cfg(feature = "futures")]
//! # extern crate futures_core_0_3 as futures_core;
//! # fn main() {
//! fn assign() {
//!     // you can assign stub!() to a variable
//!     let username = stub!(String);
//!     println!("Hello {username}")
//! }
//!
//! fn return_position() -> String {
//!     // you can use stub!() like todo!() in return position
//!     stub!()
//! }
//!
//! fn infer_type() {
//!     // you can let the compiler automatically infer the type
//!     let status = stub!();
//!     if status { println!("Success") }
//! }
//!
//! fn explicit_type() {
//!     // you can specify the type explicitly
//!     let status: bool = stub!();
//!     if status { println!("Success") }
//! }
//!
//! fn custom_message() {
//!     // you can add a custom message
//!     let status: bool = stub!("Send a request to GitHub");
//!     if status { println!("Success") }
//! }
//!
//! # #[cfg(feature = "alloc")]
//! fn impl_example() -> impl core::fmt::Display {
//!     // you can use stub!() in return position even with `impl Trait` return type
//!     // note: `impl Trait` must be written as `impl dyn Trait` due to `macro_rules!` limitation
//!     stub!(impl dyn core::fmt::Display)
//! }
//!
//! fn iter_example() -> impl Iterator<Item = u32> {
//!     // use stub_iter!() when the return type is an iterator
//!     stub_iter!()
//! }
//!
//! #[cfg(feature = "futures")]
//! fn stream_example() -> impl futures_core::stream::Stream<Item = u32> {
//!     // use stub_stream!() when the return type is a stream
//!     stub_stream!()
//! }
//!
//! fn explicit_type_with_message_example() -> u32 {
//!     // you can add
//!     stub!(u32, "Assigned to: {}", "John")
//! }
//!
//! fn explicit_type_example() -> u32 {
//!     stub!(u32)
//! }
//!
//! fn implicit_type_with_message_example() -> u32 {
//!     stub!("Assigned to: {}", "John")
//! }
//!
//! fn implicit_type_example() -> u32 {
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
//! - `stub!()` macro is intended for use during development and should be replaced with actual implementations before production use.
//! - When using `impl Trait` in return position, you must use `impl dyn Trait` in the macro invocation due to a limitation in `macro_rules!`
//! - `stub!(impl dyn Trait)` requires the `alloc` feature.
//! - `stub_stream!()` requires the `futures` feature.
//!

#![cfg_attr(not(test), deny(unused_crate_dependencies))]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::fmt::Arguments;

#[doc(hidden)]
#[macro_export]
#[cfg(feature = "alloc")]
macro_rules! _stub_impl_dyn {
    ($ty:ty) => {{
        $crate::_stub_box::<$ty>()
    }};
    ($ty:ty, $fmt:expr) => {{
        $crate::_stub_box_msg::<$ty>(::core::format_args!($fmt))
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_box_msg::<$ty>(::core::format_args!($fmt, $($args)*))
    }};
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(feature = "alloc"))]
macro_rules! _stub_impl_dyn {
    ($ty:ty) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
    ($ty:ty, $fmt:expr) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        compile_error!("stub!(impl dyn ...) requires the `alloc` feature")
    }};
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
macro_rules! stub {
    (impl $ty:ty, $fmt:expr) => {{
        $crate::_stub_impl_dyn!($ty, $fmt)
    }};
    (impl $ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_impl_dyn!($ty, $fmt, $($args)*)
    }};
    (impl $ty:ty) => {{
        $crate::_stub_impl_dyn!($ty)
    }};
    ($ty:ty, $fmt:expr) => {{
        $crate::_stub_msg::<$ty>(::core::format_args!($fmt))
    }};
    ($ty:ty, $fmt:expr, $($args:tt)*) => {{
        $crate::_stub_msg::<$ty>(::core::format_args!($fmt, $($args)*))
    }};
    ($ty:ty) => {{
        $crate::_stub::<$ty>()
    }};
    ($fmt:expr) => {{
        $crate::_stub_msg(::core::format_args!($fmt))
    }};
    ($fmt:expr, $($args:tt)*) => {{
        $crate::_stub_msg(::core::format_args!($fmt, $($args)*))
    }};
    () => {{
        $crate::_stub()
    }};
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
macro_rules! stub_iter {
    () => {
        $crate::_stub::<::core::iter::Empty<_>>()
    };
    ($fmt:expr) => {
        $crate::_stub_msg::<::core::iter::Empty<_>>(::core::format_args!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::_stub_msg::<::core::iter::Empty<_>>(::core::format_args!($fmt, $($args)*))
    };
}

/// See the crate-level documentation for the overview of this macro.
#[macro_export]
#[cfg(feature = "futures")]
macro_rules! stub_stream {
    () => {
        $crate::_stub::<$crate::_StubEmptyStream<_>>()
    };
    ($fmt:expr) => {
        $crate::_stub_msg::<$crate::_StubEmptyStream<_>>(::core::format_args!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        $crate::_stub_msg::<$crate::_StubEmptyStream<_>>(::core::format_args!($fmt, $($args)*))
    };
}

#[doc(hidden)]
#[cfg(feature = "futures")]
pub use futures_util_0_3::stream::Empty as _StubEmptyStream;

#[doc(hidden)]
pub fn _stub<T>() -> T {
    todo!()
}

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub fn _stub_box<T: ?Sized>() -> Box<T> {
    _stub::<Box<T>>()
}

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub fn _stub_box_msg<T: ?Sized>(msg: Arguments<'_>) -> Box<T> {
    _stub_msg::<Box<T>>(msg)
}

#[doc(hidden)]
pub fn _stub_msg<T>(msg: Arguments<'_>) -> T {
    todo!("{msg}")
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use alloc::string::String;
    #[cfg(feature = "alloc")]
    use core::fmt::Display;
    #[cfg(feature = "futures")]
    use futures_core_0_3::stream::Stream;

    #[test]
    #[should_panic]
    fn test_panic() {
        stub!()
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg() {
        let _: u32 = stub!(u32, "Assigned to: {}", "John");
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_msg_with_path_expression() {
        let _: u32 = stub!(u32, "Assigned to: {}", String::from("x"));
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg_with_format_expression() {
        let _: u32 = stub!(u32, concat!("Assigned to: ", "{}"), "John");
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_msg_with_named_argument() {
        let name = "John";
        let _: u32 = stub!(u32, "Assigned to: {name}", name = name);
    }

    #[test]
    #[should_panic]
    fn test_panic_iter() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!()
        }
        let _ = iter();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_iter_with_path_expression() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!("Assigned to: {}", String::from("x"))
        }
        let _ = iter();
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_iter_with_format_expression() {
        fn iter() -> impl Iterator<Item = u32> {
            stub_iter!(concat!("Assigned to: ", "{}"), "John")
        }
        let _ = iter();
    }

    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_iter_with_named_argument() {
        fn iter() -> impl Iterator<Item = u32> {
            let name = "John";
            stub_iter!("Assigned to: {name}", name = name)
        }
        let _ = iter();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic]
    fn test_panic_dyn_display() {
        fn display() -> impl Display {
            stub!(impl dyn Display)
        }
        let _ = display();
    }

    #[cfg(feature = "alloc")]
    #[test]
    #[should_panic]
    fn test_panic_dyn_iter() {
        fn iter() -> impl Iterator<Item = u32> {
            stub!(impl dyn Iterator<Item = u32>)
        }
        let _ = iter();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic]
    fn test_panic_stream() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!()
        }
        let _ = stream();
    }

    #[cfg(all(feature = "alloc", feature = "futures"))]
    #[test]
    #[should_panic(expected = "Assigned to: x")]
    fn test_panic_stream_with_path_expression() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!("Assigned to: {}", String::from("x"))
        }
        let _ = stream();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_stream_with_format_expression() {
        fn stream() -> impl Stream<Item = u32> {
            stub_stream!(concat!("Assigned to: ", "{}"), "John")
        }
        let _ = stream();
    }

    #[cfg(feature = "futures")]
    #[test]
    #[should_panic(expected = "Assigned to: John")]
    fn test_panic_stream_with_named_argument() {
        fn stream() -> impl Stream<Item = u32> {
            let name = "John";
            stub_stream!("Assigned to: {name}", name = name)
        }
        let _ = stream();
    }
}
