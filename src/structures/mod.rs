struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user(name: &String) -> User {
    User {
        name: String::from(name),
        email: String::from(name) + &String::from("@gmail.com"),
        active: true,
        sign_in_count: 1
    }
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