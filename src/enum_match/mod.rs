pub fn sample() {
    test_ip_addr_enum();
    test_message_enum();
    test_option();
    println!("{}", test_match())
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn test_ip_addr_enum() {
    let localhost_v4 = IpAddr::V4(127, 0, 0, 1);
    let localhost_v6 = IpAddr::V6(String::from("::1"));    
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
    }
}

fn test_message_enum() {
    let m = Message::Move{x: 32, y: 32};
    m.call();
}

fn test_option() {
    let x = Some(5);
    match test_option_sub(x) {
        Some(5) => println!("six"),
        _ => println!("else")
    }
    if let Some(6) = test_option_sub(x) {
        println!("six");
    }
    else {
        println!("else");
    }
}

fn test_option_sub(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn test_match() -> u32 {
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            println!("Dime!");
            10
        },
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => 0
    }
}
