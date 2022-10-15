fn stack_var() {
    {
        let s = 32;
        let s = "stack string";
        let mut s = "stack string";
        s = "const string";
        let x = 3;
        let y = x;
    }
}

fn heap_var() {
    {
        let s = String::from("heap string");
        let s1 = s;
        // println!("{}", s); // s의 소유권이 s1으로 이동되었으므로 compile error
        let s2 = s1.clone(); // 복사하면 소유권이 이동되지 않으므로 괜찮음
        println!("{} {}", s1, s2);
    }
}

fn ownership() {
    fn get_ownership() -> String {
        let s = String::from("ownership string");
        s // 리턴하면서 s의 소유권이 get_ownership()을 호출한 쪽으로 넘어감
    }

    let s = get_ownership();
    println!("{}", s);
}

fn ref_ownership(s: &String) -> usize {
    // s.push_str("impossible"); // s는 참조자이기 때문에 s를 변경하지 못함. 단순히 읽기만 가능
    s.len()
}

fn ref_mut_ownership(s: &mut String) -> usize {
    s.push_str(",possible"); // 참조자를 변경하려면 mut으로 받아야함
    s.len()
}

fn reference() {
    let s = String::from("ref string");
    println!("{} {}", ref_ownership(&s), s);

    let mut s = String::from("ref mut string");
    ref_mut_ownership(&mut s);
    println!("{}", s);

    let mut s = String::from("no ref twice");
    let s1 = &mut s;
    s1.push_str(",s1");
    let s2 = &mut s; // s를 s1에서 mutable로 참조했으나, 여기서 s2가 mutable 참조를 뺏어감. 여기 이후부터 s1 못씀
    // s1.push_str(",s1 again"); // s2에서 s를 mutable 참조하고 있으므로 compile error
    s2.push_str(",s2");
    // println!("{} {}", s, s2); // s2에서 s를 mutable 참조하고 있으므로 compile error
    s.push_str("s"); // s2로 갔던 mutable 참조를 다시 s가 가져옴. 여기 이후부터 s2 못씀
    println!("{}", s);

    let mut s = String::from("mix ref");
    let s1 = &s;
    let s2 = &s;
    println!("{} {}", s1, s2); // immutable 참조는 두 번 할 수 있음
    let s3 = &mut s; // s3가 mutable로 참조. 따라서 위 s1, s2가 가져갔던 immutable 참조는 모두 여기 이후부터 못씀
    // println!("{} {}", s1, s2);
    println!("{}", s3);
}

fn dangle() -> String { //&String {
    let s = String::from("dangle");

    // 이렇게 하면 String의 참조자를 반환한다는 것인데, String은 반환 후에 메모리에서 사라질 것이기 때문에
    // dangling pointer가 만들어질 위험이 있다. rust에서는 compile error로 막아줌
    // &s
    s
}

fn dangling_ref() {
    let s = dangle();
    println!("{}", s);
}

fn slice() {
    let s = String::from("slice this sentence");
    let slice1 = &s[..5];
    let slice2 = &s[6..];
    println!("{}", slice1);
    println!("{}", slice2);

    let slice3 = first_word(&s);
    println!("{}", slice3);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    // for (i, &item) in bytes.iter().emumerate() {
    //     if item == b' ' {
    //         return &s[0..i];
    //     }
    // }
    &s[..]
}

pub fn sample() {
    stack_var();
    heap_var();
    ownership();
    reference();
    dangling_ref();
    slice();
}
