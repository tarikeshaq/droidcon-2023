#[no_mangle]
pub unsafe extern "C" fn sync() -> *const u8 {
   let s = String::from("foo");
   s.as_bytes().as_ptr()
}

fn main() {
    let k = unsafe { sync() };
    println!("{}", *k)
}
