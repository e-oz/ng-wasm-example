use wasm_bindgen::prelude::*;

pub fn factorial(num: u128) -> u128 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}


#[wasm_bindgen]
pub fn get_factorial(num: u8) -> String {
    let mut f: u128 = 0;
    for _ in 0..10000000 {
      f = factorial(num as u128); 
    }
    f.to_string()
}