fn main() {
    let mut x = 5;
    let mut inc = || {
        x += 1;
    };
    inc();
    println!("변수 x: {}", x);

    let x = 10;
    let add = |y| x + y;
    println!("10 + 5 = {}", add(5));

    let s = String::from("Hello");
    let f = move || {
        println!("s: {}", s);
    };

    f();
    println!("s: {}", s);
}
