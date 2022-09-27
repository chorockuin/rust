pub fn sample() {
    test_stack_var();
    test_heap_var();
    test_ownership();
    test_ref();
    test_dangling_ref();
}

fn test_stack_var() {
    {
        let s = 32;
        let s = "stack string";
        let mut s = "stack string";
        let x = 3;
        let y = x;
    }
}

fn test_heap_var() {
    {
        let s = String::from("heap string");
        let s1 = s;
        // println!("{}", s);
        let s2 = s1.clone();
        println!("{} {}", s1, s2);
    }
}

fn test_ownership() {
    fn get_ownership() -> String {
        let s = String::from("ownership string");
        s
    }

    let s = get_ownership();
    println!("{}", s);
}

fn ref_ownership(s: &String) -> usize {
    // s.push_str("impossible");
    s.len()
}

fn ref_mut_ownership(s: &mut String) -> usize {
    s.push_str(",possible");
    s.len()
}

fn test_ref() {
    let s = String::from("ref string");
    println!("{} {}", ref_ownership(&s), s);

    let mut s = String::from("ref mut string");
    ref_mut_ownership(&mut s);
    println!("{}", s);

    let mut s = String::from("no ref twice");
    let s1 = &mut s;
    s1.push_str(",s1");
    let s2 = &mut s;
    s2.push_str(",s2");
    println!("{}", s);

    let mut s = String::from("mix ref");
    let s1 = &s;
    let s2 = &s;
    println!("{} {}", s1, s2);
    // let s3 = &mut s;
    // println!("{} {} {}", s1, s2, s3);
}

fn dangle() -> String { //&String {
    let s = String::from("dangle");

    // &s    
    s
}

fn test_dangling_ref() {
    let s = dangle();
    println!("{}", s);
}