mod tutorial;
mod common;
mod concurrency;
mod enum_match;
mod error;
mod ownership;
mod smart_pointer;

use std::thread;
use std::time::Duration;

fn simple_closure() {
    let x = 4;
    // x는 closure 내에서 사용하기위해 캡쳐됨
    // 그렇다면 소유권은? 마찬가지로 뺏어오던가(FnOnce), 불변으로 빌려오던가(Fn), 가변으로 빌려오던가(FnMut)
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

fn ownership_closure() {
    let x = vec![1, 2, 3];
    // move 키워드를 통해 소유권을 클로져 안으로 넘겼음(FnOnce)
    let equal_to_x = move |z| z == x;
    // 그래서 여기서 x를 참조하려고 하면 에러남
    // println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));    
}

struct Cacher<T>
    // 환경 캡쳐 정책은 불변으로 빌려오겠다는 것(Fn)
    where T: Fn(u32) -> u32 
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    // 생성자
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, // self.value에 값이 있으면 그 값 그대로 리턴
            // self.value에 값이 없으면, 클로저를 한번 실행하고 결과 값을 self.value에 업데이트 한다
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn complex_closure() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn simple_iterator() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1,2,3];
    // next()를 사용하면 반복자의 내부 상태가 변경되기 때문에 mut로 정의해야 함
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));

    let v3 = vec![1,2,3];
    let v4: Vec<_> = v3.iter().map(|x| x+1).collect(); // 이터레이터 + 클로저
    assert_eq!(v4, vec![2,3,4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

fn complex_iterator() {
}

fn functional() {
    simple_closure();
    complex_closure();
    ownership_closure();
    simple_iterator();
    complex_iterator();
}

fn main() {
    functional();
    // common::sample();
    // concurrency::sample();
    // enum_match::sample();
    // error::sample();
    // ownership::sample();
    // smart_pointer::sample();
    // tutorial::sample();
}