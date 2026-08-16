use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() {
    long_form_error_handling();
    // short_form_error_handling_expect();
    // short_form_error_handling();
    let username = read_username_from_file().unwrap();
    println!("{username}");
    let username = read_username_from_file_short().unwrap();
    println!("{username}");
    let username = read_username_from_file_very_short().unwrap();
    println!("{username}");

    // shortcut
    let username = fs::read_to_string("hello.txt").unwrap();
    println!("{username}");
}

fn long_form_error_handling() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

fn short_form_error_handling() {
    let greetig_file = File::open("hello1.txt").unwrap();
}

fn short_form_error_handling_expect() {
    let greeting_file = File::open("hello1.txt").expect("File is not found");
}

// Propagating Errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_very_short() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
