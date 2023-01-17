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

fn advanced_trait() {
}

fn advanced_type() {
}

fn advanced_function_and_closure() {
}
