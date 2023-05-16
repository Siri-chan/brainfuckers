/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! # Brainfuckers
//! Brainfuck Interpreter and `.bf`->`.c` Transpiler.
//!
//! ## Basic Usage
//! ```rs
//! use libbrainfuckers::*;
//! fn main() {
//!     // Presuming `v` is a `Vec<u8>` of Unicode Bytes
//!     let v: Vec<u8> = "++++++++++[>+++++++>+++++++++++>++++++++++<<<-]>----.>++++.>---.++++++++.<----.>---.<+++++++.>---.++++++++.------.<---.+.".bytes().collect();
//!
//!     // Either Interpret the Brainfuck Code immediately, printing output to stdout
//!     interpret(v);
//!
//!     // Or Transpile it to a `String` containing C code
//!     transpile(v); // !! IMPORTANT: If using the `cc` feature, there is a `bool` argument as well as `v` that determines if to compile the generated C code.
//! }
//! ```
//!
//! ## Modules
//! ### `lex` module
//! The `lex` module contains the code used to remove comments and turn Unicode Bytes into `libbrainfuckers::Token`s.
//!
//! ### `interp` module
//! The `interp` module contains the `run()` function used by the interpreter to run brainfuck code.
//!
//! ### `transpile` module
//! The `transpile` module contains the code to convert `Token`s to C
//!
//! #### `cc` module
//! The `transpile` module also contains the `cc` module when using the `cc` feature.  
//! The `cc` module contains code for running a C compiler on generated C code.
//!
//! ## Features:
//! ### `cc` feature
//! The `cc` feature enables experimental C compilation.
//!
//! ### `raw-mode`
//! The `raw-mode` feature is currently broken, but is intended to be used to get a single character from `stdin` using a terminal's "raw mode"
//!

pub mod interp;
pub mod lex;
pub mod transpile;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
pub enum Token {
    mvl, // <
    mvr, // >
    inc, // +
    dec, // -
    put, // .
    get, // ,

    // * jz and jnz contain the position of the matching `[` or `]` respectively
    jz(usize),  // [
    jnz(usize), // ]
    eof, // While not directly signals a brainfuck operation, signals that script end has been reached
}

/// Expects a `Vec<u8>` with unicode bytes. The easiest way to make this work is with `<String>.chars().map(|c| c as u8).collect::<Vec<u8>>();`
pub fn interpret(code: Vec<u8>) {
    interp::run(lex::tokenise(lex::trim_non_code(code).unwrap()));
}

#[cfg(not(feature = "cc"))]
pub fn transpile(code: Vec<u8>) -> Result<String, String> {
    Ok(transpile::transpile(lex::tokenise(lex::trim_non_code(
        code,
    )?)))
}

#[cfg(feature = "cc")]
pub fn transpile(code: Vec<u8>, use_cc: bool) -> Result<String, String> {
    let s = transpile::transpile(lex::tokenise(lex::trim_non_code(code)?));
    if use_cc {
        transpile::cc::compile(&s);
    }
    Ok(s)
}
