use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file(path: String) -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open(path)?;
    let _ret = f.read_to_string(&mut s)?;
    Ok(s)
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    // let result = File::open("test.txt");

    // let f = match result {
    //     Ok(f) => f,
    //     Err(err) => {
    //         panic!("파일 열기 실패: {:?}", err)
    //     },
    // };

    // let f = File::open("test.txt").unwrap();

    // let f = File::open("test.txt").expect("에러");
    
    let ret = div(1, 0);
    println!("ret: {}", ret);
    
    let ret = read_from_file(String::from("test.txt")).expect("파일 읽기 중 오류가 발생했습니다.");

    println!("파일 열기 성공");
    println!("test.txt: {}", ret);
}
