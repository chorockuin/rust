pub fn sample() {
    encapsulation();
    inheritance();
    trait_vector();
    state_pattern();
    advanced_state_pattern();
}

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) { // public method
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> { // public method
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 { // public method
        self.average
    }

    fn update_average(&mut self) { // private method
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn encapsulation() {
    let mut ac = AveragedCollection{list: vec![1,2,3,4,5,6,7,8], average: 0.0};
    println!("{}", ac.average());
    ac.add(9);
    ac.add(10);
    println!("{}", ac.average());
    ac.remove();
    println!("{}", ac.average());
}

fn inheritance() {
    /*
    상속의 목적 = 코드 재사용 + 다형성(polymorphism)
    상속이 곧 다형성(polymorphism : 다수의 타입에서 동작하도록 함)이라고 보통 생각하는데, 다형성을 지원하기 위해 상속을 사용한다는 말이 더 정확하다
    최근 언어에서 상속이 제거되고 있는 이유는 상속으로 인해 필요 이상으로 코드가 공유되는 경우가 많기 때문이다
    또한 쓸데 있는 코드/없는 코드를 구분하는 것은 매우 어려운 일이고, 변경에도 취약하다
    따라서 rust에서 상속은 지원하지 않으나 코드 재사용은 트레잇으로, 다형성은 제네릭 + 트레잇 바운드로 지원한다
    */
}

/*
Draw 트레잇만 구현하면 모두 draw() 함수를 사용할 수 있다
draw()함수를 사용할 수 있다면, Draw 트레잇이라고 생각할 수도 있는데, 이는 duck typing과 유사한 개념
*/
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("this is Button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("this is SelectBox");
    }
}

pub struct Screen {
    // Draw 트레잇을 가진 객체라면 어느 객체라도 components 백터 내에 넣을 수 있음
    // 이는 곧 runime에 판단해서 dynamic dispatch를 한다는 뜻이고, runtime 비용이 발생한다는 뜻이다
    pub components: Vec<Box<dyn Draw>>,
}

// pub struct Screen<T: Draw> {
//     // 반면에 제너릭 타입을 사용하면, components 벡터 내에는 모두 동일한 타입만 들어갈 수 있다
//     // 이는 곧 static dispatch를 한다는 뜻이고, compile time에 모두 결정되며 runtime 비용이 발생하지 않는다는 뜻이다
//     pub components: Vec<T>,
// }

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// impl<T> Screen<T>
//     where T: Draw {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

fn trait_vector() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            // Box::new(String::from("Hi")), // String은 Draw 트레잇을 구현하지 않았기 때문에 당연히 compile error
        ],
    };
    screen.run();
}

/*
트레잇 객체는 object-safe하게 만들어야 하는데 다음 두 가지 규칙을 준수해야 한다
1. 반환값의 타입이 Self(트레잇을 구현한 concret한 구현체의 타입을 가리키는 키워드)가 아니어야 함
2. 제네릭 타입 매개변수가 없어야 함
https://github.com/rust-lang/rfcs/blob/master/text/0255-object-safety.md
*/
pub trait Clone {
    fn clone(&self) -> Self;
}

// Self를 반환하는 트레잇을 사용하지 말라고하며 compile error남
// pub struct Screen2 {
//     pub components: Vec<Box<dyn Clone>>,
// }

pub struct Post {
    state: Option<Box<dyn State>>, // post의 state는 변화 가능
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // new() 하면 draft state 
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    // https://doc.rust-lang.org/stable/std/option/enum.Option.html
    pub fn request_review(&mut self) {
        // take()를 써서 Some() 안에 있는, State 트레잇을 구현한 구현체의 소유권을 s로 넘기고, state는 None으로 만든다
        if let Some(s) = self.state.take() {
            /*
            State 트레잇을 구현한 구현체 s의 request_review() 하면, s만 생성할 수 있는 새로운 State 트레잇 구현체가 만들어진다
            즉, State 트레잇 구현체마다 생성할 수 있는(전이할 수 있는) 다른 State 트레잇 구현체가 정해져있다는 뜻이다
            */
            self.state = Some(s.request_review())
        }
    }

    // request_review()와 동일한 로직이다
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    // review 요청을 했을 때 생성할(전이할) State를 각자 구현하시오 라는 뜻
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    // approve를 했을 때 생성할(전이할) State를 각자 구현하시오 라는 뜻
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // content 참조 요청
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // Draft State에서 review를 요청하면 PendingReview State를 생성한다
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    // Draft State에서 approve하면 새로운 State를 생성하지 않고, 자기자신을 리턴한다
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // PendingReview State에서 review를 요청하면 새로운 State를 생성하지 않고, 자기자신을 리턴한다
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // PendingReview State에서 approve하면 Published State를 생성한다
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    // Published State에서 review를 요청하면 새로운 State를 생성하지 않고, 자기자신을 리턴한다
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Published State에서 review를 요청하면 새로운 State를 생성하지 않고, 자기자신을 리턴한다
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    
    /*
    Published State에서 content를 요청하면 content를 출력해야 하므로, Post가 가진 content를 참조한다
    content 데이터는 State마다 갖고 있을 필요가 없고 Post만 가지고 있으면 된다.
    Post는 content에만 관여하고, State 전이에는 관여하지 않는다. State 변경에는 State만 관여한다
    결국 State만 전이(삭제 + 생성) 시켜가며 content를 출력할지 말지 결정할 수 있다.    
    이렇게 상태와 데이터를 분리시켜 결합도를 줄이고 응집력을 높이는 패턴이 state pattern이다. 
    */
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content()); // 아직 approve 되지 않았기 때문에 content를 참조해도 아무 내용도 나오지 않아야 함

    post.request_review();
    assert_eq!("", post.content()); // 아직 approve 되지 않았기 때문에 content를 참조해도 아무 내용도 나오지 않아야 함

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content()); // approve 된 이후에는 content 내용이 나와야 함
}

/*
아래 처럼 State 별 Post를 각기 다른 타입으로 정의해서 처리할 수도 있다
이해가 쉽고 코드가 단순해진다는 장점이 있으나 content가 중복된다는 점과 결합도가 올라간다는 점을 고려해서 선택하자
*/
pub struct Post2 {
    content: String,
}

pub struct DraftPost2 {
    content: String,
}

impl Post2 {
    pub fn new() -> DraftPost2 {
        DraftPost2 {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost2 {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost2 {
        PendingReviewPost2 {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost2 {
    content: String,
}

impl PendingReviewPost2 {
    pub fn approve(self) -> Post2 {
        Post2 {
            content: self.content,
        }
    }
}

fn advanced_state_pattern() {
    let mut post = Post2::new();
    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content()); // 아예 content() 메소드가 없기 때문에 compile error

    let post = post.request_review();
    // assert_eq!("", post.content()); // 아예 content() 메소드가 없기 때문에 compile error

    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
