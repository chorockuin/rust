// use std::prelude; // default import
use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn sample() {
    let magic_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input your guess: ");

/* 원래는 아래처럼 사용자 입력을 받아야 하지만, 빠른 테스트를 위해 guess 값을 fix함
        let mut guess = String::new(); // associated function(static method in c++)
        io::stdin().read_line(&mut guess).expect("fail to read line");
        let mut guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
 */
        let guess = magic_number;

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
