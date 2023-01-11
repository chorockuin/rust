fn test_box() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// 아래 정의한 Message가 차지하는 공간은 enum 값 중 가장 큰 공간을 차지하는 ChangeColor로 간주된다
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 러스트에서는 컴파일 타임에 타입이 차지하는 공간을 정확히 알 수 있어야 한다
// 아래 정의한 List 타입은 재귀적이므로 차지하는 공간을 컴파일 타임에 정확히 알 수 없다
// 따라서 컴파일 에러가 난다
// enum List {
//     Cons(i32, List),
//     Nil,
// }
// use List::{Cons, Nil};
// fn test_recursive_type() {
//     let list = Cons(1, Cons(2, Cons(3, Nil))); // 런타임에서만 차지하는 메모리 공간을 확인할 수 있다
// }

// 아래 정의한 List 타입은 재귀적인 것처럼 보이나 그렇지 않다
// Box<List>이 차지하는 공간은 usize으로 컴파일 타임에 정확히 알 수 있기 때문
// 따라서 컴파일 된다
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn test_recursive_type() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

fn test_deref() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Box는 위의 참조자와 동일하게 역참조 가능
fn test_box_deref() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// Box 흉내내기
use std::ops::{Deref, DerefMut};
struct MyBox<T>(T); // T 인스턴스 1개를 가지고 있는 구조체
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { // self를 인자로 받지 않았으므로 연관함수. MyBox::new() 형태로 사용(C++ static 메소드 비슷)
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> { // MyBox<T>에 Deref 트레잇(C++ abstract 메소드 비슷)을 입혀야 역참조 가능
    type Target = T; // 트레잇이 사용하는 연관타입을 정의한다고 함

    fn deref(&self) -> &T {
        &self.0 // 가지고 있는 T 인스턴스의 참조를 리턴
    }
}

fn test_my_box_deref() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 원래 *(y.deref()) 이렇게 해야 하는데, 역참조 연산자(*)만 쓰면 러스트에서 알아서 해줌
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn test_force_deref() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]); // m은 스마트 포인터이기 때문에 원래 스트링 슬라이스를 참조하려면 이렇게 해야 된다
    hello(&m); // 하지만 스마트 포인터의 경우 러스트에서 참조 강제를 하기 때문에 이렇게만 쓰면 됨. MyBox Deref -> String Dref
}

// 가변 참조자 트레잇도 구현 가능
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn test_drop() {
    let x = MyBox::new(String::from("test Drop"));
    // x.drop(); // 러스트에서는 강제로 drop을 메소드를 호출할 수는 없음
    drop(x); // 강제로 하려면 이렇게 해야 함
    // println!("{}", &(*x)[..]); // drop한 후에 이 코드는 컴파일 에러 남
}

// Drop 트레잇(C++ 소멸자 비슷)도 구현 가능
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop!");
    }
}

// Rc<T>는 오직 싱글 쓰레드에서만 가능
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
use std::rc::Rc;
fn test_rc() {
    let a = Box::new(Cons(5, Box::new(Cons(10, Box::new(Nil)))));
    let b = Cons(3, a);
    // let c = Cons(4, a); // a의 소유권이 b 안으로 이동했기 때문에 컴파일 에러 난다

    let x = Rc::new(RcList::Cons(5, Rc::new(RcList::Cons(10, Rc::new(RcList::Nil)))));
    println!("count after creating x = {}", Rc::strong_count(&x));
    let y = RcList::Cons(3, Rc::clone(&x)); // Rc::clone()은 참조 카운트만 증가시킴. deep copy 아니며 deep copy를 하려면 x.clone() 해야 함
    println!("count after creating y = {}", Rc::strong_count(&x));
    {
        let z = RcList::Cons(4, Rc::clone(&x));
        println!("count after creating z = {}", Rc::strong_count(&x));
    }
    println!("count after z goes out of scope = {}", Rc::strong_count(&x));
}

// Rc<T> : 동일한 데이터에 대해 복수개의 소유자 허용, 컴파일 타임에 불변 빌림만
// Box<T> : 동일한 데이터에 대해 단일 소유자만, 컴파일 타임에 불변/가변 빌림 허용

// RefCell<T> : 동일한 데이터에 단일 소유자만, 런타임에 불변/가변 빌림 허용
// 빌림 규칙은 런타임에 검사되며 잘못된 경우 panic! 발생
// 싱글 쓰레드에서만 가능
// RefCell<T> 가 불변이더라도 내부 값을 변경할 수 있으며, 이를 내부 가변성 패턴이라고 함
// C++에서 pointer는 const로 불변하게 하고, pointer가 가리키는 값은 가변하게끔 하는 것과 비슷
fn test_refcell() {
    let x = 5;
    // let y = &mut x; // x가 immutable이기 때문에 컴파일 에러남
}

// 내부 가변성 패턴 예제
pub trait Messenger {
    fn send(&self, msg: &str); // self, msg는 불변 참조자
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where T: Messenger { // 타입이 Messenger일 경우
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
                self.messenger.send("Warning: You've used up over 75% of your quota!");
            } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
                self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            // MockMessenger { sent_messages: vec![] }
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // self.sent_messages.push(String::from(message)); // self가 불변이기 때문에 컴파일 에러남. 정의가 달라지기 때문에 &mut self로 바꿀 수도 없음
            self.sent_messages.borrow_mut().push(String::from(message)); // 이때 RefCell을 활용함
            // RefCell은 한 개의 가변 참조, 여러 개의 불변 참조를 가능하게 하고, 내부적으로 참조를 카운팅 함

            let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // 여러 개의 가변 참조를 사용하려 했으므로 런타임 panic! 발생
        }
    }

    #[test]
    pub fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// 복수개의 불변 참조자를 지원하는 Rc 안에 가변 참조자를 지원하는 RefCell을 넣어두면
// 결국 복수개의 가변 참조자를 지원하도록 할 수 있음
#[derive(Debug)]
enum RcRefList {
    Cons(Rc<RefCell<i32>>, Rc<RcRefList>),
    Nil,
}
use std::cell::RefCell;
fn test_rc_refcell() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcRefList::Cons(Rc::clone(&value), Rc::new(RcRefList::Nil)));

    let b = RcRefList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = RcRefList::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// 순환 참조 예제
#[derive(Debug)]
enum CircularList {
    Cons(i32, RefCell<Rc<CircularList>>),
    Nil,
}
impl CircularList {
    fn tail(&self) -> Option<&RefCell<Rc<CircularList>>> {
        match *self {
            CircularList::Cons(_, ref item) => Some(item),
            CircularList::Nil => None,
        }
    }
}

fn test_circular_ref() {
    let a = Rc::new(CircularList::Cons(5, RefCell::new(Rc::new(CircularList::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CircularList::Cons(10, RefCell::new(Rc::clone(&a)))); // b의 tail에 a를 연결시킴

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b); // a의 tail에 b를 연결시킴
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // a->b->a->b->... 로 순환 참조 되어 있으므로, stackoverflow가 날 때까지 무한 출력
    // println!("a next item = {:?}", a.tail());
}

use std::rc::Weak;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn test_weak_ptr() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Weak<T>::upgrade()는 참조자 얻는 메소드
    // 이미 메모리 해제되어 None일 수도 있기 때문에 Option을 반환한다
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //Rc::downgrade()는 Weak<T> 참조 생성

    // leaf->parent(branch) 참조할 때 parent가 Weak<T> 덕분에 Some()으로 표시됨(만약 메모리 해제 되었다면 None일 수도 있음)
    // 따라서 순환참조 발생하지 않음
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

// Weak는 자원 해제에 관여하지 않고 단순히 참조 카운팅만 한다
fn test_weak_ptr2() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // leaf 자신 1
        Rc::weak_count(&leaf), // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]), // branch->children(leaf) Rc::clone()이므로 leaf strong + 1 = 2
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // leaf->parent(branch) 이므로 branch weak + 1 = 1

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // branch 자신 1
            Rc::weak_count(&branch), // 1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf), // 0
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // branch 없어졌으므로, None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // branch 없어졌으므로, leaf strong - 1 = 1
        Rc::weak_count(&leaf), // 0
    );
}

pub fn sample() {
    test_box();
    test_recursive_type();
    test_deref();
    test_box_deref();
    test_my_box_deref();
    test_force_deref();
    test_drop();
    test_rc();
    test_refcell();
    test_rc_refcell();
    test_circular_ref();
    test_weak_ptr();
    test_weak_ptr2();
}
