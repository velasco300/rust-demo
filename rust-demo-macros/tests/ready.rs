use rust_demo_macros::trancition;

#[trancition]
pub fn hello(s: &str) -> String {
    let a = 1;
    eprintln!("a = {}", a);
    let b = if a > 0 { a + 1 } else { a + 2 };
    println!("{},{}", s, b);
    word();
    format!("hello {}", s)
}

fn word() {
    println!("---");
}

fn main() {}
