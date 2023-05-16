/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Token;
use getkey::*;
use std::io::{self, Write};

#[cfg(feature = "raw-mode")]
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
                let n: u8 = match getkey().unwrap() {
                    Key::Char(c) => c as u8,
                    _ => panic!("Not a valid character"), //todo this should be safer than this
                };
                data[data_ptr] = n;
            }
            _ => unimplemented!(), // todo Token::get is currently poorly implemented bc im too lazy to sort out getchar
        }
        instruction_ptr += 1;
    }
    io::stdout().flush().unwrap();
}
