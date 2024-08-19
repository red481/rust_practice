fn main() {
    let number = 30;
    let long_number: i64 = 12345679123456789;
    let real = 10.22;
    let hangul_char = '러';

    println!("32비트 정수: {}", number);
    println!("64비트 정수: {}", long_number);
    println!("32비트 실수: {}", real);
    println!("문자: {}", hangul_char);

    println!("i8: MIN = {}, MAX = {}", i8::MIN, i8::MAX);
    println!("i16: MIN = {}, MAX = {}", i16::MIN, i16::MAX);
    println!("i32: MIN = {}, MAX = {}", i32::MIN, i32::MAX);
    println!("i64: MIN = {}, MAX = {}", i64::MIN, i64::MAX);
    println!("i128: MIN = {}, MAX = {}", i128::MIN, i128::MAX);

    println!("u8: MIN = {}, MAX = {}", u8::MIN, u8::MAX);
    println!("u16: MIN = {}, MAX = {}", u16::MIN, u16::MAX);
    println!("u32: MIN = {}, MAX = {}", u32::MIN, u32::MAX);
    println!("u64: MIN = {}, MAX = {}", u64::MIN, u64::MAX);
    println!("u128: MIN = {}, MAX = {}", u128::MIN, u128::MAX);

    println!("f32: MIN = {}, MAX = {}", f32::MIN, f32::MAX);
    println!("f64: MIN = {}, MAX = {}", f64::MIN, f64::MAX);
}
