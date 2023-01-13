// rust에서는 기본적으로 unit(private) test, integration(public) test, doc test를 수행함
//
// 전체 테스트
//      cargo test
// 특정 함수 테스트
//      cargo test 함수명
// 특정 문자열을 포함하는 함수는 모두 테스트(#[ignore] notation이 붙어있는 함수는 제외)
//      cargo test 함수명이 포함하는 문자열
// 다중 쓰레드로 테스트
//      cargo test --test-threads=4
// 성공하는 테스트의 출력 보기
//      cargo test -- --nocapture

pub fn sample() -> i32 {
    1024
}

fn internal_sample() -> i32 {
    2048
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)] // test cfg를 적용하라는 annotation. 이 notation을 사용하면, cargo test 시에만 compile되며, cargo build 시에는 compile되지 않음
mod tests {
    use super::*; // internal_sample() 등 상위 fn들을 사용하기 위함

    #[test]
    fn unit_test() {
        assert_eq!(2048, internal_sample());
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result // custom error message도 출력가능
        );
    }

    #[test]
    #[should_panic] // panic이 발생하는 경우를 test할 수 있음
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    // panic을 발생시키는 조건이 여러 개가 있다면, panic이 발생했다고 내가 원하는 route가 test된 것은 아닐 수 있음
    // 따라서 panic 메세지를 체크해서 명확히 내가 원하는 route로 panic이 발생했는지 test할 수 있어야 함
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_with_expected() {
        Guess::new(200);
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value); // 성공, 실패에 대한 이유를 출력해줘서 assert!()보다 좋다
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }    

    #[test]
    #[ignore]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }    
}
