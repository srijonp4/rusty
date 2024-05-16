#[allow(unused)]
struct User {
    name: String,
    age: i8,
    email: String,
    active: bool,
    sign_in_count: i32,
}

// using structs like a class
struct Rect {
    height: f32,
    width: f32,
}
impl Rect {
    fn area(&self) -> f32 {
        self.height * self.width
    }
}

fn main() {
    let user = User {
        name: String::from("Srijon"),
        age: 21,
        email: String::from("srijonp4@gmail.com"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user.name);

    let rect1 = Rect {
        width: 50.0,
        height: 20.0,
    };
    println!("{}", rect1.area());
}
