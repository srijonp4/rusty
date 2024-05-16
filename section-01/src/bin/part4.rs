fn main() {
    let str = String::from("Srijon Paul");
    println!("First name {}", get_first_name(str));
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for char in str.chars() {
        if char == ' ' {
            break;
        }
        first_name.push(char)
    }
    return first_name;
}
