use std::env;
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

pub fn sample() {
    handle_recoverable_error();
    // handle_unrecoverable_error();
}

fn handle_unrecoverable_error() {
    // backtrace 환경 변수를 1로 셋팅하면 backtrace 가능
    env::set_var("RUST_BACKTRACE", "1");
    crash_explicitly();
    crash_implicitly();
}

// 명시적으로 panic 발생시킬 수 있음
fn crash_explicitly() {
    panic!("crash and burn");
}

fn crash_implicitly() {
    let v = vec![1, 2, 3];
    v[99]; // vector의 크기는 runtime에 늘어날 수도 있기 때문에 compile time에 에러나지 않는다

    let y = [1,2,3];
    // y[99]; // array의 크기는 fix 되었으므로 out of index의 경우 compile time에 에러난다
}

fn handle_recoverable_error() {
    error();
    match_error();
    match_error_by_unwrap();
    match_error_by_expect();
    propagate_error();
    propagate_error_by_question_mark();
    propagate_error_shortly();
}

fn error() {
    // 반환 type이 Result<T, E> 임을 체크. 아래와 같이 Ok 또는 Err로 감싸져 있다
    // enum Result<T, E> {
    // Ok(T),
    // Err(E),
    let f = File::open("hello.txt");
}

fn match_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(read_f) => read_f,
        Err(ref read_e) if read_e.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(create_f) => create_f,
                Err(create_e) => {
                    panic!("There was a problem opening the file: {:?}", create_e)
                }
            }
        },
        Err(read_e) => {
            panic!("There was a problem opening the file: {:?}", read_e)
        }
    };
}

// unwrap()을 호출하면, Result가 Err로 반환될 경우, 핸들링 하지 않고 그냥 panic 발생시키겠다는 것
fn match_error_by_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

// expect()를 호출하면, Result가 Err로 반환될 경우, 핸들링 하지 않고 그냥 panic 발생시키되, 내가 원하는 메시지를 출력시키도록 하겠다는 것
fn match_error_by_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 그냥 Result<T, E>를 핸들링한 후, 다시 리턴 type을 Result<T, E> 로 만들어 리턴하겠다는 것
fn propagate_error() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    
    let mut f = match f {
        Ok(read_f) => read_f,
        Err(read_e) => return Err(read_e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?를 쓰면 그냥 Result<T, E> 형식의 리턴 값을 계속 forwarding 하겠다는 것
// 뭐 중간에 panic이 걸리면 바로 뻗겠지
fn propagate_error_by_question_mark() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn propagate_error_shortly() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 1~100 사이에 수만 갖는 값을 입력값으로 받는 프로그램이 있다고 가정할 때,
// 아래와 같이 입력값이 범위 안에 포함되지 못하면 panic을 발생시켜 생성 자체를 안해버리게 할 수도 있다
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value 
        }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}