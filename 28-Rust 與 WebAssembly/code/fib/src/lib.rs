use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n:i32) -> u64{
    if n <=0 {
        panic!("不能小於或等於零");
    }

    match n {
        1 => 1,
        2 => 1,
        3 => 2,
        _ => fib(n-1) + fib(n-2),
    }
}