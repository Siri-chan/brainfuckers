use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};

#[cfg(feature = "raw-mode")]
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}};

#[cfg(feature = "cc")]
pub mod cc;

const VER: &str = "0.1.0";

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut file: Vec<u8> = Vec::new();
    let mut should_transpile = false;
    let mut compile_transpilation = false;

    args.remove(0);
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
    let script = trim_non_brainfuck(file).unwrap();
    let tokens = tokenise(script);

    if should_transpile {
        println!("SEE BELOW FOR GENERATED C CODE:\n================================================================================");
        println!("/*\n\tBRAINFUCKERS AUTO GENERATED C CODE FROM BRAINFUCK\n*/");
        println!("{}", transpile(tokens));
    } else {
        run(tokens);
    }
}

fn tokenise(script: Vec<char>) -> Vec<(usize, Token)> {
    let mut depth: Vec<usize> = Vec::new();
    let mut output = Vec::new();

    for (i, c) in script.iter().enumerate() {
        match c {
            '[' => depth.push(i),
            ']' => {
                let j = depth.pop().unwrap();
                output.push((i, Token::jnz(j)));
                output.insert(j, (j, Token::jz(i)));
            }
            '<' => output.push((i, Token::mvl)),
            '>' => output.push((i, Token::mvr)),
            '+' => output.push((i, Token::inc)),
            '-' => output.push((i, Token::dec)),
            '.' => output.push((i, Token::put)),
            ',' => output.push((i, Token::get)),
            _ => {}
        }
    }

    output.push((script.len(), Token::eof));
    output
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq)]
enum Token {
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

fn run(tokens: Vec<(usize, Token)>) {
    let mut instruction_ptr: usize = 0;
    let mut data_ptr: usize = 0;
    let mut data: Vec<u8> = Vec::new();
    data.push(0);

    while tokens[instruction_ptr].1 != Token::eof {
        let token = &tokens[instruction_ptr];
        match token.1 {
            Token::mvl => {
                data_ptr -= 1;
            }
            Token::mvr => {
                data_ptr += 1;
                while data.len() <= data_ptr {
                    data.push(0);
                }
            }
            Token::inc => {
                data[data_ptr] += 1;
            }
            Token::dec => {
                data[data_ptr] -= 1;
            }
            Token::put => {
                print!("{}", data[data_ptr] as char);
            }
            Token::jnz(i) => {
                if data[data_ptr] != 0 {
                    instruction_ptr = i;
                }
            }
            Token::jz(i) => {
                if data[data_ptr] == 0 {
                    instruction_ptr = i;
                }
            }
            Token::put => {
                let n: u8 = getchar() as u8;
                data[data_ptr] = n;
            }
            _ => unimplemented!(), // todo Token::get is currently unimplemented bc im too lazy to sort out pulling a terminal into raw mode and shit rn
        }
        instruction_ptr += 1;
    }
    io::stdout().flush().unwrap();
}

#[cfg(feature = "raw-mode")]
fn getchar() -> char {
    enable_raw_mode();
        //todo figure out how to get char here
    disable_raw_mode();
}

#[cfg(not(feature = "raw-mode"))]
fn getchar() -> char {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input);
    return input.chars().next().unwrap();
}

fn transpile(tokens: Vec<(usize, Token)>) -> String {
    let mut code = "#include<stdio.h>\n#include<stdlib.h>\nint main(void){\nchar array[30000] = {0}; char *ptr = array;\n".to_string();
    'pile: for token in tokens {
        match token.1 {
            Token::mvl => {
                code = format!("{}--ptr;\n", code);
            }
            Token::mvr => {
                code = format!("{}++ptr;\n", code);
            }
            Token::inc => {
                code = format!("{}++*ptr;\n", code);
            }
            Token::dec => {
                code = format!("{}--*ptr;\n", code);
            }
            Token::put => {
                code = format!("{}putchar(*ptr);\n", code);
            }
            Token::get => {
                code = format!("{}*ptr = getchar();\n", code);
            }
            Token::jz(_) => {
                code = format!("{}while (*ptr) {{\n", code);
            }
            Token::jnz(_) => {
                code = format!("{}}}\n", code);
            }
            Token::eof => {
                code = format!("{}return EXIT_SUCCESS;\n}}", code);
                break 'pile;
            }
        }
    }
    code
}

fn trim_non_brainfuck(v: Vec<u8>) -> Result<Vec<char>, String> {
    let mut v1: Vec<char> = Vec::new();
    for n in v {
        match n {
            b'[' | b'>' | b'<' | b'+' | b'-' | b'.' | b',' | b']' => v1.push(n as char),
            _ => {}
        }
    }
    if v1.is_empty() {
        return Err("Fatal Error: No brainfuck code in supplied file.".to_string());
    }
    Ok(v1)
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
