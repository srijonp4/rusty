#[allow(unused)]

fn find_first_a(s: &String, c: char) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == c {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let str = String::from("Hello, World!");
    let res = find_first_a(&str, 'p');
    match res {
        Some(index) => {
            println!("character H is found at {}", index);
        }
        None => {
            println!("Not Found : -1");
        }
    }
}
