use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn sample() {
    // handle_unrecoverable_error();
    // handle_recoverable_error();
}

fn handle_unrecoverable_error() {
    env::set_var("RUST_BACKTRACE", "1");
    crash_explicitly();
    // crash_implicitly();
}

fn crash_explicitly() {
    panic!("crash and burn");
}

fn crash_implicitly() {
    let v = vec![1, 2, 3];
    v[99];
}

fn handle_recoverable_error() {
    error();
    // match_error();
    // match_error_by_unwrap();
    // match_error_by_expect();
    // propagate_error();
    // propagate_error_by_question_mark();
    // propagate_error_shortly();
}

fn error() {
    let f = File::open("hello.txt");
}

fn match_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(read_f) => read_f,
        Err(ref read_e) if read_e.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(create_f) => create_f,
                Err(create_e) => {
                    panic!("There was a problem opening the file: {:?}", create_e)
                }
            }
        },
        Err(read_e) => {
            panic!("There was a problem opening the file: {:?}", read_e)
        }
    };
}

fn match_error_by_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn match_error_by_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn propagate_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(read_f) => read_f,
        Err(read_e) => return Err(read_e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn propagate_error_by_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn propagate_error_shortly() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value 
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
