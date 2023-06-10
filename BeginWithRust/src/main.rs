
// use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret  = rand::thread_rng().gen_range(0..=100);
    
    loop{
        println!("Enter your input ");

        let mut guess = String::new();
        
        let read = std::io::stdin();

        read.read_line(&mut guess)
        .expect("fail ");

        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("u guess :{guess}");

        match guess.cmp(& secret)
        {
            Ordering::Less => println!("too low !!"),
            Ordering::Greater => println!("too big !!"),
            Ordering::Equal => 
            {
                println!("well done");
                break;
            }
        }
    }
}

