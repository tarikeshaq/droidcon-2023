fn f() -> &'static i32 {
    let a = 3;
    return &a;
}

fn main() {
    println!("a: {}", *f());
}