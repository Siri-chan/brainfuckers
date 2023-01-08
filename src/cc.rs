/* This File will be used for interface with the CC Package, and can be disabled. */

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
