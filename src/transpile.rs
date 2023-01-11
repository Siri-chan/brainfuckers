/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Token;

#[cfg(feature = "cc")]
pub mod cc;

pub fn transpile(tokens: Vec<(usize, Token)>) -> String {
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
