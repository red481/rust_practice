fn main() {
    let s: &str = "Hello 러스트!";

    println!("문자열: {}", s);

    let slice: &str = &s[0..5];
    println!("슬라이스: {}", slice);

    let s: &str = " Hello Rust ";
    println!("{}", s.trim());
    println!("{}", s.to_lowercase());
    println!("{}", s.to_uppercase());

    let mut s = String::from("Hello");
    println!("{}", s);
    s.push_str(" Rust!");
    println!("{}", s);
}
