fn main() {
    let x: i32 = 128;
    println!("{}", x);
    println!("---------------------");
    let is_male = true;
    let is_above_18 = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }
    println!("---------------------");
    // There are two ways of doing strings in rust.This is the easier one
    let greeting = String::from("hello world");
    println!("{}", greeting);

    let char = greeting.chars().nth(0);

    println!("{}", char.unwrap())
}
