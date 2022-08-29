const const_var: u32 = 100000;

fn main() {
    println!("{}", const_var);

    test_mutation();
    test_shadow();
    test_type();
    println!("{}", test_func(2));
    test_condition();
    test_loop();
}

fn test_mutation() {
    let x = 5;
    // x = 6;

    let mut y = 5;
    y = 6;
}

fn test_shadow() {
    let x = 5;
    let x = x + 1;
    let x = x + 2;
    println!("{}", x);

    let spaces = " ";
    let spaces = 32;
    println!("{}", spaces);
}

fn test_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);
    // println!("{}", a[128]);
}

fn test_func(x: i32) -> i8 {
    let y: i8 =  {
        let x = 3;
        x + 1
    };
    y
}

fn test_condition() {
    let x = 3;
    // if x {        
    // }
    if x < 5 {
    }
    else if x < 3 {        
    }
    else {
    }

    if x != 3 {
    }

    let y = if x < 3 {
        5
    } else {
        6
        // "string"
    };
}

fn test_loop() {
    let mut x = 0;
    loop {
        println!("increase x -> {}", x);

        x = x + 1;
        if x > 3 {
            break
        }
    }

    while x != 0 {
        println!("decrease x -> {}", x);

        x = x - 1;
    }

    let a = [10, 20, 30, 40, 50];
    for e in a.iter() {
        println!("for iter {}", e);
    }

    for i in (1..4).rev() {
        println!("for {}", i);
    }
}