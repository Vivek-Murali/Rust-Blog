use funcs::*;
use std::path::PathBuf;

fn main() {
    println!("Hello, world! {}",GREETING);
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");
    println!("{}",build_full_name(&john));
    println!("{:?}",safe_division(45.0, 3.0));
    if read_file_contents(PathBuf::from("data/PSD Q&A.txt")).is_ok() {
        println!("{:?}",read_file_contents(PathBuf::from("data/PSD Q&A.txt")).unwrap());
    }
    if read_file_contents(PathBuf::from("non-existent-file.txt")).is_err() {
        println!("The program reported an error for the file that doesn't exist.");
    }
}
