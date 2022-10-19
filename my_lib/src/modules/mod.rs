mod b;
mod c;

mod a {
    pub fn func_a() {
        println!("func_a()");
    }
}

fn basic() {
    a::func_a();
    b::func_b();
    b::bb::func_bb(); // 원래 경로를 참고하면 이렇게 참조해서 호출해야 하지만,
    b::func_bb(); // b.rs에 pub use bb::func_bb; 를 넣어놨기 때문에 이렇게 호출해도 된다
    c::func_c();
}

use a::func_a;
use b::func_b;
use b::func_bb; // b.rs에 pub use bb::func_bb; 를 넣어놨기 때문에 이렇게 써도 된다
use c::func_c;

fn use_modules() {
    func_a();
    func_b();
    func_bb(); // 실제로는 b모듈 밑에 bb가 있으나, b.rs에 pub use bb::func_bb; 를 넣어놨기 때문에 이렇게 호출해도 된다
    func_c();
}

pub fn sample() {
    basic();
    use_modules();
}
