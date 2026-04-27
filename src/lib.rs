//! # BetterStdin
//! `better_stdin` is a simple and lightweight input utility library inspired by Python `input()`
//! function.
//!
//! It is heavily WIP and only has 1 function right now, but I aim at adding more functionality and stability in
//! the future.
//!
//! ## Getting started
//! Just add:
//! ```
//! better_stdin = "0.1.0"
//! ```
//! to your Cargo.toml, and
//! ```
//! use better_stdin::prelude::*;
//! ```
//! at the top of your file.
//!
//! The only function this library provides (for
//! now) is [`input`].

/// This prelude module only exports [`input`] for now. I am planning to add more functions
/// later.
pub mod prelude {
    pub use crate::input;
}

use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::fmt::Debug;

/// Reads user input and parses it to a requested type.
/// Panics on any error (more robust error handling is planned).
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
/// You can also read to String without parsing (this still does trim user input though):
/// ```no_run
/// use better_stdin::prelude::*; 
/// /* --snip-- */
/// let user_string: String = input("Enter a string: ");
/// ```
///
/// Note that you _cannot_ parse input to &str because it doesn't implement FromStr trait. If you
/// really need to read a &str, then you can convert it manually:
/// ```no_run
/// use better_stdin::prelude::*;
/// /* --snip-- */
/// let temp_string: String = input("Enter a string: ");
/// let string_slice = temp_string.as_str(); // You can now use string_slice as user-inputted &str
/// ```
pub fn input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{prompt}");
    io::stdout().flush().expect("Failed to flush stdout"); // Print the prompt

    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read line"); // Read user input into input_str

    input_str = input_str.trim().to_string(); // Clean input_str of leading/trailing whitespace
    input_str.parse().expect("Invalid input") // Return input_str parsed into T type wrapped in a result
}
