/*
컴파일 타임에 동시성 문제를 발견할 수 있도록 하는 것이 목표
1. 쓰레드 생성 기초
2. 쓰레드 간 메세지 전달
3. 각 쓰레드에서의 공유 데이터 접근
4. 동시성 관련 Sync, Send 트레잇
*/
use std::thread;
use std::time::Duration;
fn base() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // context switching
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn join() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn closure() {
    fn main() {
        let v = vec![1, 2, 3];
    
        // spawned thread에서 v값을 사용하려고 함
        // main thread, spawned thread에서 v가 공유되므로 당연히 문제 발생의 여지가 있음
        // 이를 위해 move 키워드를 사용하며, v의 소유권을 spawned thread로 이동 시켜버림
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        // drop(v);
    
        handle.join().unwrap();
    }
}

use std::sync::mpsc;
fn channel() {
    // channel을 생성하면 Sender, Receiver를 갖고 있는 튜플 객체 반환
    let (tx, rx) = mpsc::channel();

    // spawned thread는 Sender를 move로 넘겨받아 "hi" 문자열 전송
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(5000));
        let val = String::from("hi");
        // send하면서 val의 소유권은 main thread로 move됨
        tx.send(val).unwrap();
        // println!("val is {}", val); // 따라서 val은 유효하지 않기 때문에 컴파일 에러남
    });

    println!("receiving...");
    // main thread는 Receiver를 가지고 수신 대기
    let received = rx.recv().unwrap(); // 동기 대기
    // let received = rx.try_recv().unwrap(); // 비동기 대기
    println!("Got: {}", received);
}

fn send_vector_vals() {
    let (tx, rx) = mpsc::channel();
    let tx_clone = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx.recv() 하지 않고 rx를 반복자처럼 다룸
    // channel이 닫히면 반복도 종료 됨
    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {
    base();
    join();
    closure();
    channel();
    send_vector_vals();
}
