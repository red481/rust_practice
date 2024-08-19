use std::io;


fn main() {
    let arr = [1, 2, 3, 4, 5];

    for a in arr {
        print!("{},", a);
    }

    println!("");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    for i in 0..arr.len() {
        print!("{}, ", arr[i]);
    }

    println!("");

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[5] = 7;

    println!("arr[5] = {}", arr[5]);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    println!("숫자를 입력해주세요.");
    let mut read = String::new();
    
}
