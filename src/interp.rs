use crate::Token;
use std::io::{self, Write};

#[cfg(feature = "raw-mode")]
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}};

pub fn run(tokens: Vec<(usize, Token)>) {
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
            Token::get => {
                let n: u8 = getchar() as u8;
                data[data_ptr] = n;
            }
            _ => unimplemented!(), // todo Token::get is currently poorly implemented bc im too lazy to sort out pulling a terminal into raw mode
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