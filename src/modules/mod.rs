mod b;
mod c;

mod a {
    pub fn a() {
        println!("a()");
    }
}

use b::b;
use c::c;
use a::a;

pub fn sample() {
    a();
    b();
    c();
}
