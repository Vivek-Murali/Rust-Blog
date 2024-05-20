use core::f64;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

pub const GREETING: &'static str = "Hallo, Rust library here!";

pub struct Person {
    pub first: String,
    pub middle: Option<String>,
    pub last: String,
}

pub fn build_full_name(person: &Person) -> String {
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    if let Some(middle) = &person.middle {
        full_name.push_str(middle);
        full_name.push_str(" ");
    }

    full_name.push_str(&person.last);
    full_name
}

/* Safe Division with Result Struct */


#[derive(Debug)]
pub struct DivisionByZeroError;

pub fn safe_division(number:f64,divisor:f64) -> Result<f64,DivisionByZeroError>{
    if divisor == 0.0{
        Err(DivisionByZeroError)
    }else{
        Ok(number/divisor)
    }
}

// File operation w/d Result Strcut

pub fn read_file_contents(path: PathBuf) -> Result<String, Error> {
    let mut string = String::new();

    // Access a file at a specified path
    // ---------------------------------
    // TODO #1:
    // - Pass variable to `file` variable on success, or
    // - Return from function early if there's an error
    let mut file: File = match File::open(path) {
        Ok(file_handle) =>file_handle,
        Err(io_error) => return Err(io_error)
    };

    // Read file contents into `String` variable with `read_to_string`
    // ---------------------------------
    // Success path is already filled in
    // TODO #2: Return from function early if there's an error
    match file.read_to_string(&mut string) {
        Ok(_) => &string,
        Err(io_error) => return Err(io_error)
    };

    // TODO #3: Return `string` variable as expected by function signature
    Ok(string)
}