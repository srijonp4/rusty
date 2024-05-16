// functions

fn main() {
    let a = 4;
    let b = 41;

    println!("{}", do_stuff(a, b))
}

fn do_stuff(a: i32, b: i32) -> i32 {
    return a + b;
}
