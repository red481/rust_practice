fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut result = [0; 10];
    
    result[1] = 1;
    
    for i in 2..10 {
        result[i] = result[i-1] + result[i-2];    
    }

    println!("fibonacci using iteration from 0 to 9 result is : {}", result[9]);

    let result2 : u32 = fibonacci(8);
    println!("fibonacci using recursion function by 9 result is : {}", result2);
}

fn fibonacci(x : u32) -> u32 {
    if x == 0 {
        return 0;
    } else if x == 1 {
        return 1;
    } else {
    fibonacci(x-1) + fibonacci(x-2)
    }
}


#[test]
fn fibo_test() {
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
}