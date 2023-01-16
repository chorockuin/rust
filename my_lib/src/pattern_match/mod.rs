pub fn sample() {
    if_let();
    while_let();
    basic_let();
    for_pattern();
    function_parameters();
    refutable_pattern();
    literals_matching();
    named_variable_matching();
    multiple_pattern();
    range_matching();
    destruct_struct();
    destruct_enum();
    destruct_nested_struct_enum();
    destruct_struct_tuple();
    ignore_value_in_pattern();
    create_refer_using_ref();
    match_guard();
    at_binding();
}

fn basic_let() {
    // let PATTERN = EXPRESSION;
    let x = 5;
    let (x,y,z) = (1,2,3);
}

fn if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // match favorite_color {
    //     Some(color) => println!("Using your favorite color, {}, as the background", color),
    //     _ => (),
    // }
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color); // 위 주석처리한 코드와 동일. if let을 사용해 간략화
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age { // '= age'에서 age는 Result<u8, _> 이다.
    // } else if let Ok(age) = age && age > 30 { // age가 u8로 재정의 되었으므로, 'age > 30' 표현은 compile error
    if age > 30 { // 여기서 age는 u8이다
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_pattern() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }    
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn refutable_pattern() {
    let x = 5; // irrefutable(반증 불가능한) pattern. always success

    let option_value = Some(4);
    // let Some(some_value) = option_value; // refutable(반증 가능한) pattern. option_value가 None일 경우 반증됨. compile error

    // 함수의 매개변수, let 구문, for 루프등은 반증 불가능한 패턴만 허용. 그러나 아래와 같이 if let에는 허용
    if let Some(some_value) = option_value { //refutable(반증 가능한) pattern이나 허용됨
        println!("{}", some_value);
    }
}

fn literals_matching() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn named_variable_matching() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn multiple_pattern() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_matching() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn destruct_struct() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }    
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn destruct_enum() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {x} and in the y direction {y}"
            );
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {r}, green {g}, and blue {b}",
        ),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn destruct_nested_struct_enum() {
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change color to hue {h}, saturation {s}, value {v}"
        ),
        _ => (),
    }
}

fn destruct_refer() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    
    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
}

fn destruct_struct_tuple() {
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });
}

fn foo(_: i32, y: i32) { // ignore value using _
    println!("This code only uses the y parameter: {}", y);
}

struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

fn ignore_value_in_pattern() {
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => { // ignore value using nested _
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    let y = 10; // 미사용 경고 발생
    let _x = 5; // 변수명을 _로 시작하면 미사용 경고를 피할 수 있음

    let s = Some(String::from("Hello!"));
    if let Some(_s) = s {
        println!("found a string");
    }
    // println!("{:?}", s); // s의 값이 _s에 bind되었기 때문에, s는 소유권을 잃고 compile error

    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s); // _만 쓰면, s의 값이 _에 bind되지 않기 때문에, s는 소유권을 빼앗기지 않음

    let origin = Point3 { x: 0, y: 0, z: 0 };
    match origin {
        Point3 { x, .. } => println!("x is {}", x), // x외에 무시
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => { // 모호하므로 compile error
    //         println!("Some numbers: {}", second)
    //     },
    // }    
}

fn create_refer_using_ref() {
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name), // 참조 사용
        None => (),
    }
    println!("robot_name is: {:?}", robot_name); // 소유권이 살아있으므로 compile ok

    let mut robot_name = Some(String::from("Bors")); // mut String으로 선언
    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"), // mut 참조 사용하여 String 변경
        None => (),
    }    
    println!("robot_name is: {:?}", robot_name); // 소유권 살아있으므로 compile ok
}

fn match_guard() {
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // Some(x) 형태이고, x가 5보다 작은 경우 match
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

enum Message3 {
    Hello { id: i32 },
}

fn at_binding() {
    let msg = Message3::Hello { id: 5 };

    match msg {
        Message3::Hello { id: id_variable @ 3..=7 } => { // @을 사용해 3...7 값을 id_variable에 binding
            println!("Found an id in range: {}", id_variable)
        },
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message3::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

