use std::env;
use std::process;

mod lib;

pub fn sample() {
    basic();
    let config = lib::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}

fn basic() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() > 1 {        
        println!("{} {}", &args[0], &args[1]);
    }
}
