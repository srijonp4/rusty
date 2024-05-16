#[allow(unused)]
#[allow(dead_code)]
use core::alloc;
use std::fs;

/* enum Result<T, E> {
    Ok(T),
    Err(E),
} */
fn main() {
    let res = fs::read_to_string("example.txt");
    match res {
        Ok(file_content) => {
            println!("{}", file_content.trim_end());
        }
        Err(err) => {
            println!("error happened : {}", err);
        }
    }
    println!("handelled exception");
}
