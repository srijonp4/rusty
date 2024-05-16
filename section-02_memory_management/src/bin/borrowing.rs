/*

The Rules of References

- At any given time, you can have either one mutable - reference or any number of immutable references.
- References must always be valid.

*/

fn main() {
    let str = String::from("Hello");
    let str2 = String::from("World");
    let mut concatinated_str = String::new();
    concat(&mut concatinated_str, &str, &str2);
    println!("Concatinated String : {}", concatinated_str)
}

fn concat(result: &mut String, part1: &String, part2: &String) {
    result.push_str(part1);
    result.push_str(" ");
    result.push_str(part2);

    // result.push_str(part1).push(' ').push_str(part2); //this will not compile
    /* In Rust, the push_str method modifies the String in place and does not return a value that can be chained further. To chain multiple push_str calls, you can use the &mut self return type from the method. However, you cannot chain push_str calls directly because each call consumes the &mut self reference, making it unavailable for further chaining. */
}
