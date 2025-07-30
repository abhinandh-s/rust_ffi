use std::ffi::CString;
use std::os::raw::c_char;

// External function declarations - need unsafe in newer Rust versions
unsafe extern "C" {
    fn hello_from_c();
    fn greet_person(name: *const c_char);
    fn add_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    println!("Rust FFI Example");
    println!("================");

    // Call simple C function
    unsafe {
        hello_from_c();
    }

    // Call C function with string parameter
    let name = CString::new("Rustacean").expect("CString::new failed");
    unsafe {
        greet_person(name.as_ptr());
    }

    // Call C function that returns a value
    let result = unsafe {
        add_numbers(15, 27)
    };
    println!("15 + 27 = {} (calculated in C)", result);

    println!("FFI demonstration complete!");
}
