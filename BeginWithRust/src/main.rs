use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Enter your input ");

    let secret :i32 = rand::thread_rng().gen_range(0..=100);

    println!("this is the number : {secret}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess).expect("fail ");
    println!("u guess :{guess}");
    
}
