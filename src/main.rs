use std::io;
use std::string::String;

fn main() {

    println!("Simplest template manager");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("teste: {}", &guess);


}
