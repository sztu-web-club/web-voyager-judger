use std::fs::File;
use std::io::Write;
use std::process::Command;

pub fn compile(code: &str) -> Result<String, String> {
    let mut file = File::create("main.cpp").unwrap();
    file.write_all(code.as_bytes()).unwrap();
    let output = Command::new("g++")
        .arg("-O2")
        .arg("main.cpp")
        .arg("-o")
        .arg("main")
        .output()
        .unwrap();
    if !output.status.success() {
        let err = String::from_utf8(output.stderr).unwrap();
        println!("{}", err);
        return Err(err);
    }
    Ok(String::from("main"))
}