use c_wrap_func_wrapper::wrap_function;

#[wrap_function]
fn testing_add(one: i32, two: i32) -> i32 {
    one + two
}


fn main() {
    println!("Hello, world!");
}
