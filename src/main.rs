use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Good luck, Duck.");
    let secret = rand::thread_rng().gen_range(1,101);
    loop {

        println!("Please input your guess:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("you guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win");
                break;
                },
           }
    }


}
