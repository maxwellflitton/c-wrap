#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use c_wrap_func_wrapper::wrap_function;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn testing_add(one: i32, two: i32) -> i32 {
    one + two
}
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
