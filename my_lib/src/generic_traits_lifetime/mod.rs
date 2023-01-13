pub fn sample() {
    generic_basic();
    trait_basic();
    non_generic();
    generic_trait();
    lifetime();
}

// generic은 type generalization. c++의 template과 동일한 개념
struct Point<T> { // generic type T
    x: T,
    y: T,
}

impl<T> Point<T> { // method에서 generic type T사용
    fn get_x(&self) -> &T {
        &self.x
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn generic_basic() {
    let integer = Point{x: 5, y: 10}; 
    let float = Point{x: 1.0, y: 4.0};
    // let mix = Point { x: 1, y: 4.0 }; // compile error, T는 일관된 type이어야 함
    let mix = Point2{x: 1, y: 4.0};
    let p = Point{x: 10, y: 20};
    println!("p.x = {}", p.get_x());

    let p1 = Point2{x: 5, y: 10.4};
    let p2 = Point2{x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2); // p1과 p2의 type을 섞었음
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Option::Some(5); // compile 후에 Option<i32>가 만들어지고
    let float = Option::Some(5.0); // Option<f64>가 만들어지는 것이기 때문에 perfomance에는 영향 없다. 다만 binary가 커지겠지
}

fn non_generic_largest(list: &[i32]) -> i32 { // &[i32]에 fix되지 않고, generic하게 사용하려면?
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn non_generic() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = non_generic_largest(&numbers);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let numbers = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = non_generic_largest(&numbers);
    println!("The largest number is {}", result);
    assert_eq!(result, 6000);
}

fn generic_trait_largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T { // 사용할 trait(미리 구현되어 있는)을 명시함
    let mut largest = list[0]; // Copy trait을 사용한다고 명시했기 때문에 T type 값을 복사할 수 있음
    for &item in list.iter() {
        if item > largest { // std::cmp::PartialOrd trait을 사용한다고 명시했기 때문에 T type 값을 비교할 수 있음
            largest = item;
        }
    }
    largest
}

fn generic_trait() {
    let numbers = vec![34, 50, 25, 100, 65];
    let result = generic_trait_largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];
    let result = generic_trait_largest(&chars);
    println!("The largest char is {}", result);
}

// trait은 c++의 pure virtual method만 가진 abstract class, java의 interface와 비슷한 개념
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub trait Summarizable {
    fn summary(&self) -> String;
    fn summary2(&self) -> String { // default implementation
        String::from("(Read more...)")
    }
}

impl Summarizable for NewsArticle { // trait implementation
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    // summary2의 default implementation을 그대로 사용한다면 아무것도 안해도 됨
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet { // trait implementation
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summary2(&self) -> String { // re-implementation
        String::from("(Read more...summary2 re-implementation)")
    }
}

pub fn notify<T: Summarizable>(item: T) { // trait + generic
    println!("Breaking news! {}", item.summary());
}

fn trait_basic() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {} {}", tweet.summary(), tweet.summary2());
    notify(tweet);
}

// 함수에 lifetime 시그니처를 생략할 경우 암시적으로 다음과 같은 규칙을 따름
// 1. 참조자인 함수의 파라미터들은 각각 고유한 lifetime 시그니처를 갖음
// 2. 만약 함수의 파라미터가 1개라면 lifetime도 1개이고, 리턴 파라미터의 lifetime도 그에 맞춰 동일하게 결정됨
// 3. 여러 개의 함수 파라미터가 있으나, 함수가 메소드(&self 파라미터를 갖는 경우)라면 리턴 파라미터의 lifetime도 그에 맞춰 동일하게 결정됨
// 4. 1~3 규칙에도 불구하고 lifetime을 결정할 수 없으면, compile error냄
// 모든 string literal은 static lifetime을 가지고, 'static 이라는 시그니처를 사용해 표현할 수 있음

// fn longest(x: &str, y: &str) -> &str { // compile error. return 값(x or y)의 lifetime이 불명확하기 때문
fn longest<'my_sign>(x: &'my_sign str, y: &'my_sign str) -> &'my_sign str { // 이렇게 'my_sign라는 시그니처를 만들어서 모든 파라미터에 동일하게 달아주면, 모두 동일한 lifetime을 갖는다고 명시하는 것. 따라서 lifetime이 명확해지고 compile pass
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> { // 구조체를 정의할 때 lifetime을 명시할 수도 있음
    part: &'a str, // 'a 시그니처를 사용해 part라는 구조체 멤버가 ImportantExcerpt 구조체와 운명공동체임(같은 lifetime을 가짐)을 명시했음
}

fn lifetime() {
    let r;
    {
        let x = 5;
        // r = &x; // compile error, block을 벗어나면 x는 dead
    }
    let x = 5;
    r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence }; // first_sentence와 ImportantExcerpt는 같은 block내에서 생성되었으므로 같은 lifetime임
}
