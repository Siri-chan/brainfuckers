/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Token;

pub fn trim_non_code(v: Vec<u8>) -> Result<Vec<char>, String> {
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

pub fn tokenise(script: Vec<char>) -> Vec<(usize, Token)> {
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
