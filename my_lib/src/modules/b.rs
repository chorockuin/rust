pub fn func_b() {
    println!("func_b()");
}

pub mod bb {
    pub fn func_bb() {
        println!("func_bb()")
    }
}

pub use bb::func_bb;