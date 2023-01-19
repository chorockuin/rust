pub fn sample() {
    unsafe_rust();
    advanced_lifetime();
    advanced_trait();
    advanced_type();
    advanced_function_and_closure();
}

unsafe trait Foo { // unsafe한 내용을 담고 있는 트레잇
}

unsafe impl Foo for i32 { // unsafe한 내용을 구현한 트레잇 구현체
}

unsafe fn dangerous() { // unsafe한 내용을 담고 있는 함수
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc; // mutable 전역 참조자를 변경하는 것은 unsafe한 일임
    }
}

fn unsafe_rust() {
    let mut num = 5;
    let r1 = &num as *const i32; // num 변수를 참조하는 raw pointer
    let r2 = &mut num as *mut i32; // 같은 num 변수를 mutable 참조하는 mutable raw pointer. 일반 참조자를 쓰면 소유권 문제 때문에 compile error가 나야 하지만, raw pointer를 쓰면 compile error 안남

    unsafe {
        *r2 = 4; // 이렇게 막 값을 변경해버릴 수가 있음
        println!("r1 is: {}", *r1); // 4
        println!("r2 is: {}", *r2); // 4
    }

    let address = 0x012345usize;
    let r = address as *const i32; // 임의의 주소(0x012345)를 가리키는 raw pointer

    unsafe {
        dangerous(); // unsafe 함수를 호출할 때는 반드시 unsafe block 안에서 해야 함
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x012345usize;
    let r = address as *mut i32;
    let slice = unsafe {
        std::slice::from_raw_parts_mut(r, 10000) // 프로그램 실행도중 유효하지 않은 memory address를 참조한다면 당연히 문제 발생. 조심히 써야 함
    };

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3)); // unsafe block을 사용하여 C의 abs()함수 호출
    }

    println!("name is: {}", HELLO_WORLD); // HELLO_WORLD가 immutable 하므로 괜찮음
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER); // COUNTER가 mutable하므로 자원 경쟁 상황에 안전하지 않다. 따라서 unsafe block에 두어 명시적으로 의도한 것이라고, 괜찮다고 선언해야 함
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    
    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..]) // 동일 block 내 mut 참조는 1개만 있을 수 있기 때문에, 첫번째 인자에서 mut 참조 이후, 두번째 mut 참조는 compile error
    // 위 문제를 해결하기 위해 unsafe한 rust code를 사용함
    let ptr = slice.as_mut_ptr();
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid), std::slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() { // C코드에서 이 rust function을 호출할 수 있음
    println!("Just called a Rust function from C!");
}

/*
라이프타임 서브타이핑 (subtyping): 한 라이프타임이 다른 라이프타임보다 오래 사는 것을 보장하기
라이프타임 바운드: 제네릭 타입을 가리키는 참조자를 위한 라이프타임 명시하기
트레잇 객체 라이프타임의 추론: 컴파일러는 어떻게 트레잇 객체의 라이프타임을 추론하며 언제 이들을 명시할 필요가 있는지에 대하여
*/

/*
struct Context<'a>(&'a str);

struct Parser<'a> { // Parser는 str 참조자를 갖는 Context 참조자를 갖는 구조체이므로 이렇게 str lifetime을 명시해야 함. 그렇지 않으면 compile error
    context: &'a Context<'a>,
}

impl<'a> Parser<'a> {
    // fn parse(&self) -> Result<(), &str> { // 이렇게 lifetime을 명시하지 않으면 디폴트로 parse, &self, &str의 lifetime은 암시적으로 'a가 됨
    fn parse<'a>(&'a self) -> Result<(), &'a str> { // 암시적으로는 이렇게 되어있는 것과 같음
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    /*
    Parser와 context는 함수 내에서 생성되었으므로, 함수가 종료되면 lifetime이 끝남
    그런데 Parser의 parse() 메소드는 결국 context의 참조를 리턴하므로, 함수가 종료되더라도 리턴값(context)가 유효해야함
    따라서 Parser 내 context의 lifetime을 Parser의 lifetime이 끝나더라도 살아 있도록 정의해 줘야 함
    그렇지 않으면 compile error 발생
    */
    Parser { context: &context }.parse()
}
*/

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> { // 's의 lifetime이 'c보다 길어야 한다고 선언함
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse() // Parser가 생성되었다가 죽더라도 context는 살아있을거라고 위에서 lifetime을 명시했기 때문에 compile error가 발생하지 않음
}

struct Ref<'a, T: 'a>(&'a T); // 제너릭 타입 T에 대한 lifetime을 명시할 것(lifetime bound)
struct StaticRef<T: 'static>(&'static T); // static lifetime

trait Red { }

struct Ball<'a> {
    diameter: &'a i32, // diameter가 'a라는 lifetime을 갖고, Ball 구조체도 'a라는 lifetime을 갖음
}

impl<'a> Red for Ball<'a> { }

fn advanced_lifetime() {
    let num = 5;
    let obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>; // Ball의 인스턴스를 트레잇 객체 Box<Red>로 사용하길 원함

    /*
    트레잇 객체의 기본 라이프타임은 'static
    &'a Trait 혹은 &'a mut Trait을 쓴 경우, 트레잇 객체의 기본 라이프타임은 'a
    단일 T: 'a 구절을 쓴 경우, 트레잇 객체의 기본 라이프타임은 'a
    여러 개의 T: 'a 같은 구절들을 쓴 경우, 기본 라이프타임는 없음; 우리가 명시적으로 써야함
    */
}

pub trait Iterator {
    type Item; // 연관타입(입력 필드에 입력 힌트가 표시되는 타입을 플레이스 홀더 타입이라고 함. 여기에서는 next 메소드의 반환 값이 Item의 의미를 갖을 것이니 참고해서 트레잇을 구현하라고 힌트를 주고 있음)
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}

// 이와 같이 연관타입으로 트레잇을 구현하면 타입 명시를 type Item = u32로 한번만 하면 된다. next() 메소드 구현에서 명시할 필요가 없다
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// 반면에 제네릭을 사용하면 next 메소드를 구현할 때 리턴 타입인 Option<T>에 타입 T를 명시해야 한다
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// rust는 임의의 연산자 오버로딩을 지원하지 않지만, std::ops에 있는 트레잇을 구현함으로써 연산자를 오버로딩 할 수 있음
impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/*
std::ops::Add 트레잇은 아래와 같이 생겼는데
제네릭 타입인 RHS를 정의하지 않으면 Self, 즉 자기자신의 타입을 디폴트로 사용한다
위의 Point 트레잇 구현이 그 예시다
*/
// trait Add<RHS=Self> {
//     type Output;

//     fn add(self, rhs: RHS) -> Self::Output;
// }

/* 
반면에 아래 예시는 Add의 타입을 명시하고 있다
따라서 Millimeters 트레잇 구현체의 Add는 Millimeters 대신 Meters를 제네릭 타입으로 사용하고 있다
*/
struct Millimeters(u32);
struct Meters(u32);

impl std::ops::Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

// Human은 Pilot, Wizard, Human 모두의 fly() 메소드를 구현하고 있다. 어떻게 모호함을 극복하는가?
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// 또한 이렇게 연관함수를 가진 트레잇의 메소드 명이 같을 경우 어떻게 모호함을 극복하는가?
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// OutlinePrint 트레잇은 Display 트레잇의 to_string() 메소드를 사용하기 위해 이렇게 정의함. Display 트레잇은 OutlinePrint의 슈퍼트레잇이라고 함
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Point 구조체에서 OutlinePrint 트레잇의 디폴트 구현을 사용하고 싶어서 이렇게 코드를 만들었으나, 이것만 만들면 안되고
impl OutlinePrint for Point {}

// OutlinePrint는 Display 트레잇을 사용하고 있기 때문에, Display 트레잇의 반드시 구현해야 하는 fmt 메소드도 역시 구현해야 함
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/* 
만약 Vec에 대해서 Display 트레잇을 구현하게끔 하고 싶다면,
원칙적으로는 Vec나 Display 둘 중 하나는 현재 crate에 속해 있어야 함
그러나 그렇지 않다. 이럴 경우 어떻게 트레잇을 구현할 것인가?
아래와 같이 Vec의 튜플 구조체를 가진 뉴타입을 정의하여 구현할 수 있다
이를 뉴타입 패턴이라고 함
*/
struct Wrapper(Vec<String>); // Wrapper라는 튜플 구조체가 Vec를 가지고 있음. Wrapper는 현재 crate 내에 있으므로 트레잇 구현이 가능함

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn advanced_trait() {
    assert_eq!(Point{x:1, y:0} + Point{x:2, y:3}, Point{x:3, y:3});
    let x = Millimeters(1000) + Meters(1);

    // 동일한 이름의 메소드를 호출하는 모호함을 아래와 같이 극복함
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // 동일한 이름의 메소드가 연관함수일 경우에는 모호함을 극복하기 위해 아래와 같이 완전 정규화 문법을 사용해야 함
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // 뉴타입 패턴을 사용하여 Vec에 Display 트레잇의 fmt 메소드를 구현했다
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);    
}

// pub trait Write {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error>; // Result<usize, std::io::Error> 타입을 매번 이렇게 쓰는게 귀찮다
//     fn flush(&mut self) -> Result<(), std::io::Error>;

//     fn write_all(&mut self, buf: &[u8]) -> Result<(), std::io::Error>;
//     fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> Result<(), std::io::Error>;
// }

// std::io에 type Result<T> = Result<T, std::io::Error>와 같이 정의되어 있으므로 이걸 활용하면 편하다
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    fn flush(&mut self) -> std::io::Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()>;
    fn write_fmt(&mut self, fmt: std::fmt::Arguments) -> std::io::Result<()>;
}

fn advanced_type() {
    type Kilometers = i32; // type alias라고 함
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);    

    type Thunk = Box<dyn Fn() + Send + 'static>; // 길게 정의된 타입을 간단하게 줄여서 쓸 수 있음
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // loop {
    //     let mut guess = String::new();
    //     std::io::stdin().read_line(&mut guess).expect("fail to read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         /*
    //         guess는 반드시 u32 타입이어야만 하는데, 어떻게 반환 값을 continue를 써도 문제가 없을까?
    //         conitnue는 부정타입 !를 가지기 때문이다. !은 값을 가지지 않는다는 뜻이다.
    //         guess는 결국 Ok(num)의 u32 타입과 Err(_)의 부정타입 !를 가진다는 뜻이고
    //         부정타입은 값이 없다는 뜻이므로 guess는 항상 u32 타입으로 fix되기 때문에 문제가 발생하지 않는다 
    //         */
    //         Err(_) => continue,
    //     };    
    // }

    // 아래 panic!도 부정타입 !를 가지기 때문에 위의 continue와 동일한 이유로 문제가 발생하지 않는다
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    /* 
    rust에서는 같은 타입이라면 크기가 모두 같아야 하고, 컴파일 타임에 그 크기가 결정되어야 함
    아래 str 타입은 런타임에 s1이 만들어질 때에야 비로소 크기가 정해지고(동적인 크기의 타입: dynamically sized type)
    심지어 같은 str 타입임에도 s1, s2의 크기가 다르기 때문에 rust에서는 허용되지 않고 컴파일 에러가 발생한다
    */
    // let s1: str = "Hello there!";
    // let s2: str = "What's going on?";

    /*
    rust에서 트레잇은 dynamically sized type이기 때문에 위와 같은 이유로
    &Trait 또는 Box<Trait> 또는 Rc<Trait>과 같은 형태로 트레잇 객체를 사용해야 함 
    */

    /* 
    아래 제네릭 함수는 <T> 부분이 <T: Sized>라고 작성된 것처럼 동작함
    Sized는 트레잇인데, T의 크기가 정해져있어야 한다고 제약을 거는 트레잇이다.
    즉, 모든 제네릭에 사용되는 타입은 컴파일 타임에 그 크기를 알야아만 한다는 것이다
    */    
    fn generic<T>(t: T) {
        // --snip--
    }

    /*
    만약 제네릭 타입의 크기를 알지 못할 수도 있는 경우에는 아래와 같이
    Sized앞에 ?를 붙여서 그것을 표현할 수 있고,
    크기를 모를 경우에는 크기가 정해진 참조나 포인터를 사용해야 한다
    아래에서는 &T 라고 참조를 사용했다
    */    
    fn generic2<T: ?Sized>(t: &T) {
        // --snip--
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

/* 
클로저 뿐만 아니라 함수를 그냥 함수 포인터 타입으로 인자로 넘길 수도 있음. fn은 트레잇이 아니고 타입이다
함수 포인터는 Fn, FnMut, FnOnce 트레잇을 모두 구현하고 있다. 따라서 클로저를 인자로 원하는 함수에 클로저 대신 함수 포인터를 넘겨도 됨
*/
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn advanced_function_and_closure() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string()) // 클로저를 map의 인자로 넣었음
        .collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string) // 이렇게 클로저 대신 함수 포인터를 map의 인자로 넘겨도 됨
        .collect();

    /*
    아래 코드는 리턴값으로 클로저를 반환하고 있는데, 클로저는 트레잇에 의해 표현된다.
    트레잇은 동적인 크기를 가지고 있기 때문에 앞에서 언급했듯이 참조나 포인터로만 사용해야 한다.
    따라서 아래 코드는 컴파일 에러를 발생시킨다
    */
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // 따라서 아래와 같이 포인터로 사용해야 한다
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
