pub mod lex;
pub mod interp;
pub mod transpile;

#[cfg(feature = "cc")]
pub mod cc;

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

/// Expects a Vec<u8> with unicode bytes. The easiest way to make this work is with `<String>.chars().map(|c| c as u8).collect::<Vec<u8>>();`
pub fn interpret (code: Vec<u8>) {
    interp::run(lex::tokenise(lex::trim_non_code(code).unwrap()));
}

#[cfg(not(feature = "cc"))]
pub fn transpile (code: Vec<u8>, use_cc: bool) -> Result<String, String> {
    Ok(transpile::transpile(lex::tokenise(lex::trim_non_code(code)?)))
}

#[cfg(feature = "cc")]
pub fn transpile (code: Vec<u8>, use_cc: bool) -> Result<String, String> {
    let s = transpile::transpile(lex::tokenise(lex::trim_non_code(code)?));
    if use_cc {
        cc::compile(&s);
    }
    Ok(s)
}