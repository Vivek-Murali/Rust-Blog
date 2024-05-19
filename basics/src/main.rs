use basics::GREETING;
use basics::*;
use basics::build_full_name;

fn main() {
    println!("Hello, world! {}",GREETING);
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Oliver")),
        last: String::from("Smith"),
    };
    assert_eq!(build_full_name(&john), "James Oliver Smith");
    println!("{}",build_full_name(&john))
}
