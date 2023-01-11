const const_var: u32 = 100000; // not allowed using mut

pub fn sample() {
    println!("{}", const_var);

    mutation();
    shadow();
    types();
    println!("{}", func(2));
    condition();
    loops();
}

fn mutation() {
    let x = 5;
    // x = 6; // compile error. 기본적으로 immutable

    let mut y = 5; // mutable 명시
    y = 6;
}

fn shadow() {
    let x = 5;
    println!("{:p} {}", &x, x);

    let x = x + 1;
    println!("{:p} {}", &x, x);

    let x = x + 2;
    println!("{:p} {}", &x, x);

    let spaces = " ";
    let spaces = 32;
    println!("{}", spaces);
}

fn types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple
    println!("{} {} {}", tup.0, tup.1, tup.2); // tuple indexing
    let a = [1, 2, 3, 4, 5]; // array
    println!("{}", a[0]); // array indexing
    // println!("{}", a[128]); // compile error, invalid index
}

fn func(x: i32) -> i8 {
    let y: i8 = { // function
        println!("{:p} {}", &x, x);
        
        let x = 3; // x를 정의했으므로, 파라미터로 받은 x는 무시됨
        println!("{:p} {}", &x, x);
        x + 1 // return
    };
    y // return
}

fn condition() {
    let x = 3;
    // if x { // compile error, if 뒤에는 bool type만 위치할 수 있음. x는 integer
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
        // "string" // compile error, if와 else의 return type은 일치해야 함
    };
}

fn loops() {
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

    for i in (1..4).rev() { // reverse
        println!("for {}", i);
    }
}