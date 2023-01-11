struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn sample() {
    let user1 = User {
        name: String::from("chorockuin"),
        email: String::from("chorockuin@gmail.com"),
        active: true,
        sign_in_count: 1
    };

    let user2 = create_user(&String::from("dongseok"));
    println!("{}, {}", user2.name, user2.email)
}

fn create_user(name: &String) -> User {
    User {
        name: String::from(name),
        // + 연산자는 string literal만 지원하기 때문에 String이 아닌, &String을 써야 함
        email: String::from(name) + &String::from("@gmail.com"),
        active: true,
        sign_in_count: 1
    }
}
