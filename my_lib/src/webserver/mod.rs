use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// FnBox 트레잇은 함수 포인터를 가지고 있으며, call_box를 호출하면 가지고 있는 함수 포인터에 해당하는 함수를 호출함
trait FnBox {
    fn call_box(self: Box<Self>);
}

// F는 FnOnce() 타입이므로 F는 파라미터들의 소유권을 빼앗아오기때문에 한번만 호출할 수 있음
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

// Job은 threadsafe 하게 소유권을 이동시킬 수 있고(Send) + 전역 lifetime을 갖는('static) + FnBox 타입의 포인터다
type Job = Box<dyn FnBox + Send + 'static>;

// 메세지는 Job 생성과 종료
enum Message {
    NewJob(Job),
    Terminate,
}

// Worker는 Worker ID와 thread handle을 가지고 있음
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    // Worker ID와 receiver의 소유권을 안전하게 전달 받는다
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) ->
        Worker {

        let thread = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job.call_box(); // Message에 묻어온 Job에 해당하는 함수를 실행시킴(Job이 가진 FnBox의 함수 포인터 호출)
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 채널을 통해 sender와 receiver 튜플을 받아오는데
        let (sender, receiver) = mpsc::channel();
        // receiver에는 mutex를 씌우고, threadsafe하게 소유권을 이동시킬 수 있도록 하여 재정의 한다
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            /*
            정수 ID값과 receiver의 참조자를 복사해서 갖고 있는 Worker를 만들어 Vector에 차례로 넣는다
            receiver의 참조자를 threadsafe하게 복사해서 넘겼으므로 Worker 내부에서 receiver를 문제없이 사용할 수 있다
            */
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender,
        }
    }

    // F는 한 번만 호출 될 수 있고(FnOnce : 캡쳐한 파라미터의 소유권을 한 번만 가져올 수 있으므로 호출도 한 번만 할 수 있음), 쓰레드 간 소유권을 이동시킬 수 있고(Send), 그리고 전역의 lifetime을 갖음('static)
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static
    {
        /*
        ThreadPool.execute() 하게 되면
        파라미터 넘어온 F타입의 함수에 포인터를 씌워 Job 타입으로 만들고, 만든 Job을 Message에 담아 sender로 보낸다
        해당 메세지는 Worker들 중 하나의 receiver에서 안전하게(mutex+arc) 처리할 것이고
        결국 Message > Job > F로 감싸져 있는 구성에서, 실제 수행 대상인 F의 함수포인터가 실행된다
        */
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

pub fn sample() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

     let mut file = File::open(filename).unwrap();
     let mut contents = String::new();

     file.read_to_string(&mut contents).unwrap();

     let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();
}