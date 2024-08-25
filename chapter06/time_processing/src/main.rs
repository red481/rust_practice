use std::time::SystemTime;
use std::thread::sleep;
use std::time::Duration;
use chrono::{ Utc, Local, FixedOffset };
use chrono::offset::TimeZone;

#[test]
fn test_chrono_formatting() {
    let now = Local::now();

    println!("{}", now.format("%Y-%m-%d"));

    println!("{}", now.format("%H:%M:%S"));

    println!("{}", now.format("오늘은 %A, %B %d, %Y. 현재 시간은 %H:%M:%S."));
}

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

#[test]
fn test_duration() {
    
    let start = Duration::new(1, 0);
    let adder = Duration::new(1, 0);
    
    let sum = start + adder;
    let minus = start - adder;

    println!("Duration 간 덧셈 결과: {:?}", sum);
    println!("Duration 간 뺄셈 결과: {:?}", minus);

}

#[test]
fn test_nanosec() {
    let duration = Duration::new(1, 123_000_000);

    let sec = duration.as_secs();
    let nano = duration.subsec_nanos();

    println!("{}초 {}나노초", sec, nano);
}

#[test]
fn test_computate_timezone() {
    let now_utc = Utc::now();
    println!("UTC 시각: {}", now_utc);

    let now_local = Local::now();
    println!("로컬 시각: {}", now_local);

    let seoul_offset = FixedOffset::east(9 * 3600);
    let seoul = seoul_offset.from_utc_datetime(&now_utc.naive_utc());
    println!("한국 시각: {}", seoul);
}

fn main() {
    println!("Hello, world!");
}
