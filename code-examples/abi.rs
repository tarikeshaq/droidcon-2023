#[no_mangle]
fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[no_mangle]
fn caller() {
    let a = 1;
    let b = 2;
	 let c = add(a, b);
    println!("c: {c}")
}

fn main() {
    caller()
}