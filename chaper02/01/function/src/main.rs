fn main() {
    let ret = add(2, 4);
    println!("2+4={}", ret);

    let x = 1;
    let y = 2;
    let ret = {
        x + y
    };

    println!("{}+{}={}", x, y, ret);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
