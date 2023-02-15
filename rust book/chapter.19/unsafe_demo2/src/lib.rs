#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!")
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}
