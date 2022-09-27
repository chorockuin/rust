pub fn sample() {
    ip_addr_enum();
    message_enum();
    option();
    println!("{}", match_sample())
}

// 다른 type을 섞어서 enum으로 묶을 수 있음
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn ip_addr_enum() {
    let localhost_v4 = IpAddr::V4(127, 0, 0, 1);
    let localhost_v6 = IpAddr::V6(String::from("::1"));    
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

// enum 내부에 메소드를 구현할 수 있음
impl Message {
    fn call(&self) {
    }
}

fn message_enum() {
    let m = Message::Move{x: 32, y: 32};
    m.call();
}

// 보통 함수를 호출하면 반환값을 받아서 그대로 사용하게 마련이다
// 그런데 그 함수가 가끔 유효하지 않은 값(예를 들면 Null 같은...)을 리턴한다면? 당연히 사용하는 측에서 리턴 값을 체크하고 핸들링 해줘야 한다
// 그러나 안해도 딱히 compile error가 나지는 않는다. 이는 안전하지 않은 코드가 될 가능성을 열어두는 것이다
// 이런 문제점을 해결하기 위해 Option을 사용한다
// Option은 함수의 반환 type을 None또는 Some으로 강제하는 type이다. 실제 사용자가 원하는 리턴 '값'은 Some()안에 들어있다
// 즉, 함수를 사용하는 측에서 반환값을 직접 사용할 수는 없다. 반드시 Some()에서 꺼내써야 한다
// 그렇게 되면 사용자는 Some()에서 값을 꺼내 쓰면서, None에 대한 처리도 반드시 고민하게 된다.(처리하는 것은 여전히 사용자에게 달려있긴 하지만..)
// 즉, Option은 사용자에게 None의 경우를 핸들링 하라고 type차원에서 강제하는 효과를 갖는다
fn option() {
    let x = Some(5);
    match option_sub(x) {
        Some(5) => println!("six"),
        _ => println!("else")
    }
    if let Some(6) = option_sub(x) {
        println!("six");
    }
    else {
        println!("else");
    }
}

fn option_sub(x: Option<i32>) -> Option<i32> {
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

// 중첩된 enum
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn match_sample() -> u32 {
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
