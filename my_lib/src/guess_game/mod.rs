use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn sample() {
    let magic_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input your guess: ");
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("you guessed: {}", guess);
    
        match guess.cmp(&magic_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
