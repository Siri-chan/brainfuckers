/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

/* This File will be used for interface with the CC Feature, and can be disabled. */

use std::{env, fs::File, io::Write};

pub fn compile(c_code: &String) {
    use std::process::Command;

    println!("ww");
    let temp_directory = env::temp_dir();
    let temp_file = temp_directory.join("bf.c");
    let mut file = File::create(&temp_file).unwrap();
    writeln!(&mut file, "{}", c_code).unwrap();

    if cfg!(target_os = "windows") {
        todo!("Cannot yet compile C code on Windows")
    } else {
        Command::new("cc")
            .args([temp_file.as_os_str()])
            .spawn()
            .expect("cc command failed to start");
    }
    //unimplemented!()
}
