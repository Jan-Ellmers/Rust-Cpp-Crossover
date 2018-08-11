use std::env::current_exe;
use std::process::Command;

static PROGRAM_DIRECTORY: &str = "Rust-Cpp-Crossover";

fn main () {
    //compile c++ code
    let output = Command::new("g++")
        .args(&["-shared", "-fpic", "foo.cpp", "-o", "target/libtest.so"])
        .output()
        .expect("failed to execute process");

    //print the error if we have one
    let error = String::from_utf8(output.stderr).unwrap();
    if error != "".to_owned() {
        panic!("C++ compile error: \n{}", error);
    }


    //should not go wrong
    let path = current_exe().unwrap();
    let mut path = path.to_string_lossy().into_owned();

    //find the Program directory in the path
    let string = format!("{}/target", PROGRAM_DIRECTORY);
    let mut index = path.rfind(&string).unwrap_or_else(|| panic!("Can not find program directory"));
    index += string.len();

    //shrink to Program directory
    path.truncate(index);

    println!(r"cargo:rustc-env=LD_LIBRARY_PATH={}", path);
    println!(r"cargo:rustc-link-search=native={}", path);

}