fn main() {
    let s = String::from("Hello");
    let s = push_str(s);
    println!("{}", s);

    let mut ss = String::from("Hello");
    push_str_borrewed(&mut ss);
    println!("{}", ss);

    let mut x = 10;
    let y = x;

    println!("x : {} y : {}", x, y);

    x = 20;
    println!("x : {} y : {}", x, y);

    let mut s1 = Student{ age: 10 };
    let s2 = s1;

    println!("s1: {}, s2: {}", s1.age, s2.age);

    s1.age = 20;

    println!("s1: {}, s2: {}", s1.age, s2.age);
}

fn push_str(mut s: String) -> String {
    s.push_str(" Rust!");
    s
}

fn push_str_borrewed(s: &mut String) {
    s.push_str(" Rust!");
}

#[derive(Copy, Clone)]
struct Student {
    age: i32,
}