/*

[What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
*/

/* // 01.Passing stack Variables inside functions

fn main() {
        let x = 1; // crated on stack
        let y = 3; // created on stack
    println!("{}", sum(x, y));
    println!("Hello, world!");
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b;
    return c;
}

 */

/*// 02.Scoping variables in the same fn

fn main() {
    let x = 1; // crated on stack
    {
        let y = 3; // created on stack
    }

    println!("{}", y); // throws error
}
*/

/*
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; //now the current owner is s2
                 // println!("{}", s1); // This line would cause a compile error because ownership has been moved.
    println!("{}", s2)
    // #NOTE: when the owner dies the heap gets deallowcated
}
*/

/* // 04. Passing strings (heap variables) to functions as args
fn main() {
    let my_string = String::from("hello");
    // takes_ownership(my_string);
    // println!("{}", my_string); // This line would cause a compile error because ownership has been moved.

    // fix : clone the string
    takes_ownership(my_string.clone());
    println!("my_string : {}", my_string);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}
 */

/* 05. But what if you want to pass the same string over to the function? You don’t want to clone it, and you want to return back ownership to the original function?
You can either do the following - */

fn main() {
    let s1 = String::from("hello");
    let s2 = takes_ownership(s1);
    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    return some_string; // return the string ownership back to the original main fn
}
