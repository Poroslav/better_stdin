//! # `BetterStdin`
//! `better_stdin` is a simple and lightweight input utility library inspired by Python `input()`
//! function.
//!
//! It is heavily WIP right now, but I aim at adding more functionality and stability in
//! the future.
//!
//! ## Getting started
//! Just add:
//! ```
//! better_stdin = "0.1.3"
//! ```
//! to your Cargo.toml, and
//! ```
//! use better_stdin::prelude::*;
//! ```
//! at the top of your file.

/// I am planning to add more functions
/// later.
pub mod prelude {
    pub use crate::input;
    pub use crate::input_result;
}

use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

/// Reads user input and parses it to a requested type.
///
/// # Examples
///
/// Basic usage (note that you are **required** to annotate the type):
/// ```no_run
/// use better_stdin::prelude::*;
/// /* --snip-- */
/// let num: i32 = input("Enter an integer: ");
/// let float: f64 = input("Enter a float: ");
/// ```
///
/// You can also read to `String` without parsing (this still does trim user input though):
/// ```no_run
/// use better_stdin::prelude::*;
/// /* --snip-- */
/// let user_string: String = input("Enter a string: ");
/// ```
///
/// Note that you _cannot_ parse input to `&str` because it doesn't implement `FromStr` trait. If you
/// really need to read a `&str`, then you can convert it manually:
/// ```no_run
/// use better_stdin::prelude::*;
/// /* --snip-- */
/// let temp_string: String = input("Enter a string: ");
/// let string_slice = temp_string.as_str(); // You can now use string_slice as user-inputted &str
/// ```
///
/// # Panics
/// Panics on any error, such as failing to flush, read line, and parse input (for custom error handling, use [`input_result`]).
#[must_use]
pub fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout"); // Print the prompt

    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line"); // Read user input into input_str

    input_str = input_str.trim().to_string(); // Clean input_str of leading/trailing whitespace
    input_str.parse().expect("Invalid input") // Return input_str parsed into T type
}

/// Reads user input and return Result with parsed input.
///
/// # Examples
/// Basic usage is similar to [`input`] but you also need to handle possible errors (if you only
/// need panic on invalid input, use [`input`]):
/// ```no_run
/// let a: i32;
/// loop {
///     match input_result::<i32>("Enter an integer: ") {
///         Ok(val) => {
///             a = val;
///             break;
///         }
///         Err(_) => {
///             println!("Invalid input. Try again.");
///         }
///     }
/// }
/// println!("{a}");
/// ```
///
/// # Panics
/// Panics only on unrecoverable errors (failing to flush and read line).
///
/// # Errors
/// Returns `Err` if user input can't be parsed to requested type.
pub fn input_result<T>(prompt: &str) -> Result<T, <T as FromStr>::Err>
// No must_use here as
// Result already implies it
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout"); // This part is same as input()

    let mut input_str = String::new();
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    input_str = input_str.trim().to_string();
    input_str.parse() // Instead of panicking on invalid input, return Result (with handleable
    // error)
}
