pub fn sample() {
    vector();
    string();
    hashmap();
}

fn vector() {
    let v: Vec<i32> = Vec::new(); // type을 반드시 명시해야 함
    let mut v = Vec::new(); // type을 명시하지 않으면 이후 코드에서 type을 추론할 근거가 있어야 함
    v.push(5); // type을 i32로 추론
    
    let mut v = vec![1,2,3,4,5]; // vec! 매크로를 사용하면 type 추론하여 vector를 만듬
    let thrid = &v[2]; // 직접 access
    // let third = &v[1000]; // 직접 access인데 out of index이면 panic이 발생함
    
    let third = v.get(1000); // get()을 통해 access하면 Option<>을 돌려준다. 즉, out of index와 같은 error handling이 가능함
    match third {
        Some(item) => println!("{}", item),
        None => println!("error!")
    }
    
    let first = &v[0]; // vector의 참조를 따 놓고
    v.push(6); // vector를 확장하면, vector의 memory reallocation이 발생하는 경우 따놓은 참조 값이 유효하지 않게 될 수 있기 때문에
    // println!("{:p}", first); // 참조를 사용하는 코드에서 compile error가 난다

    let v = vec![100,32,57]; // immutable vector
    for i in &v { // immutable ref
        println!("{}", i);
    }

    let mut v = vec![100,32,57]; // mutable vector
    for i in &mut v { // mutable ref
        *i += 50;
    }
    println!("{}", &v[0]); // 150

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // enum을 활용해서, vector의 각 요소가 다른 type을 가지도록 만들 수 있음
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i)
    }
}

fn string() {
    let mut s = String::new();
    println!("{}", s);

    let data = "initial contents"; // &str은 string literal에 대한 참조자
    println!("{}", data);

    let s = data.to_string();
    println!("{}", s);

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(&s2);

    let mut s = String::from("lo");
    s.push('l');
    
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 더해지는 string은 &str 형식을 사용해야 함. &String하면 역참조 강제에 의해 &str 하는 것과 같음
    // println!("{}", s1); // s3 = s1 + &s2는 s1의 소유권을 뺏은 후, s2를 읽어서 더하고, 그 소유권을 s3에 넘긴 것이기 때문에 여기에서 s1은 더이상 유효하지 않음. 이는 performance 때문

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // 가독성이 떨어지므로

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");    
    let s = format!("{}-{}-{}", s1, s2, s3); // 이와 같이 formatting을 함
    println!("{} {} {} {}", s, s1, s2, s3); // formatting을 하면 소유권이 유지됨

    let s1 = String::from("hello");
    // let h = s1[0]; // compile error가 나는데, 이는 String이 가변길이 문자인 utf8으로 인코딩 되어 있어 indexing의 의미가 모호하기 때문

    let hello = "Здравствуйте";
    let s = &hello[0..4]; // 굳이 indexing 한다면, range slicing으로. 그러나 runtime panic이 발생할 수 있으므로 주의할 것

    // String을 바라보는 3가지 방식
    // 1. byte
    for c in "초록거인".bytes() {
        println!("{}", c);
    }

    // 2. unicode scalar value
    for c in "초록거인".chars() {
        println!("{}", c);
    }

    // 3. grapheme cluster
}

fn hashmap() {
}