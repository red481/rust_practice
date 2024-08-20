use std::io;   

fn main() {
    let condition = true;
    if condition == true {
        println!("조건이 참입니다.");
    } else {
        println!("조건이 거짓입니다.");
    }

    let condition = true;
    let ret = if condition == true {
        String::from("조건이 참입니다.")
    } else {
        String::from("조건이 거짓입니다.")
    };

    println!("result : {}", ret);

    println!("------match------");

    let var = 1;
    match var {
        1 => println!("하나"),
        2 => println!("둘"),
        _ => println!("기타"),
    }

    let var = 1;
    let ret = match var {
        1 => String::from("하나"),
        2 => String::from("둘"),
        _ => String::from("기타"),
    };

    println!("ret={}", ret);

    loop {
        println!("숫자를 입력해주세요. 0을 입력하면 종료합니다.");
        let mut read = String::new();
        io::stdin().read_line(&mut read).unwrap();
        let val: i32 = read.trim().parse().unwrap();

        if val == 0 {
            break;
        }

        println!("입력={}", val);
    }

    let arr = [1, 2, 3, 4, 5];
    for a in arr {
        print!("{}, ", a);
    }

    println!();
    
    for a in 0..5{
        print!("{}, ", a);
    }

    let mut counter = 0;

    print!();
    
    while counter < 5 {
        print!("{}, ", counter);
        counter += 1;
    }
}
