use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_comparing_times() {
    let start = SystemTime::now();
    sleep(Duration::from_secs(1));
    let end = SystemTime::now();

    if end > start {
        println!("시간 비교 가능");
    }
}

#[test]
fn test_computate_times() {
    let now = SystemTime::now();
    let after = now + Duration::from_secs(3);

    println!("현재시간: {:?}", now);
    println!("+3초: {:?}", after);
}

#[test]
fn test_measure_interval_times() {
    let tm = SystemTime::now();

    println!("1초 대기");
    sleep(Duration::from_secs(1));

    match tm.elapsed() {
        Ok(elapsed) => {
            println!("대기 시간: {}.{}초", elapsed.as_secs(), elapsed.subsec_millis());
        }
        Err(e) => {
            println!("오류 발생: {:?}", e);
        }
    }
}

#[test]
fn test_check_current_time() {
    let now = SystemTime::now();

    println!("{:?}", now);
}

fn main() {
    println!("Hello, world!");
}
