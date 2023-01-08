/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use libbrainfuckers::*;

use std::{
    env,
    fs::File,
    io::{self, Read},
};

const VER: &str = "0.1.0";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file: Vec<u8> = Vec::new();
    let mut should_transpile = false;
    #[cfg(feature = "cc")]
    let mut compile_transpilation = false;

    'search: loop {
        if args.is_empty() {
            read_stdin(&mut file).unwrap();
            if file.is_empty() {
                panic!("Fatal Error: No Valid File Specified, nor code passed through STDIN")
            }
            break 'search;
        }
        let arg = args.pop().unwrap(); // We know for certain this should never panic, as we check if `args` is empty since last mutation
        let mut chars = arg.chars();
        if chars.next().unwrap() == '-' {
            // todo this may panic as Unwrap isn't certain string won't be empty
            while let Some(c) = chars.next() {
                match c {
                    'h' => {
                        display_help();
                        return;
                    }
                    'v' => {
                        display_version();
                        return;
                    }
                    'c' => should_transpile = true,
                    #[cfg(feature = "cc")]
                    'C' => compile_transpilation = true,

                    _ => unimplemented!("unknown argument"), // !fixme we should DEFINITELY NOT panic on an unknown argument
                }
            }
        } else {
            match read_file(&arg, &mut file) {
                Err(e) => eprintln!(
                    "Non-Fatal Error: Reading File Failed on File <{}>. Rust Error Readout: {}",
                    arg, e
                ),
                _ => {}
            };
            if file.is_empty() {
                eprintln!("Non-Fatal Error: File <{}> has no contents.", arg);
                continue 'search;
            }
            break 'search;
        }
    }
    let script = lex::trim_non_code(file).unwrap();
    let tokens = lex::tokenise(script);

    if should_transpile {
        println!("SEE BELOW FOR GENERATED C CODE:\n================================================================================");
        println!("/*\n\tBRAINFUCKERS AUTO GENERATED C CODE FROM BRAINFUCK\n*/");
        let s = transpile::transpile(tokens);
        println!("{}", s);
        #[cfg(feature = "cc")]
        if compile_transpilation {
            cc::compile(&s);
        }
    } else {
        interp::run(tokens);
    }
}

fn read_file(filename: &String, buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut file = File::open(filename)?;
    file.read_to_end(buffer)?;
    Ok(())
}

fn display_help() {
    unimplemented!("only god can help you now")
}

fn display_version() {
    println!("++++++++++[>+++++++>+++++++++++>++++++++++<<<-]>----.>++++.>---.++++++++.<----.>---.<+++++++.>---.++++++++.------.<---.+.");
    println!("Brainfuckers: (Version {})", VER);
}

fn read_stdin(buffer: &mut Vec<u8>) -> io::Result<()> {
    let mut stdin = io::stdin();
    stdin.read_to_end(buffer)?;
    Ok(())
}
