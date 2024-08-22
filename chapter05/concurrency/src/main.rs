use std::fs::File;
use std::io::{BufReader, BufRead};
use std::thread;
use std::sync::mpsc;
use tokio;
use tokio::time;
use std::time::Duration;
use futures::executor::block_on;
use std::io;
use std::sync::Mutex;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_thread_create() {
    let handle = thread::spawn(|| {
        println!("스레드에서 실행");
    });

    handle.join().unwrap();
}

#[test]
fn test_file_read() {
    let handle = thread::spawn(|| {
        let file = match File::open("file.txt") {
            Ok(f) => f,
            Err(e) => {
                eprintln!("error occured in opening file.txt. error is : {}", e);
                return;
            }
        };

        let reader = BufReader::new(file);
        for line in reader.lines() {
            let txt = line.unwrap();
            println!("{}", txt);
        }
    });

    match handle.join() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("스레드 내부에서 오류가 발생했습니다. {:?}", e);
        }
    };
}

#[test]
fn test_communication_between_threads() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut sum = 0;

        for i in 1..101 {
            sum = sum + i;
        }
        
        tx.send(sum).unwrap();
    });

    let sum = rx.recv().unwrap();

    println!("1부터 100까지의 합: {}", sum);
}

#[test]
fn test_communication_multithreads() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx1);

    thread::spawn(move || {
        let mut sum = 0;
        for i in 1..51 {
            sum = sum + i;
        }
        tx1.send(sum).unwrap();
    });

    thread::spawn(move || {
        let mut sum = 0;
        for i in 51..101 {
            sum = sum + i;
        }
        tx2.send(sum).unwrap();
    });
    let mut sum = 0;
    for val in rx {
        println!("수신: {}", val);
        sum = sum + val;
    }
    println!("1부터 100까지의 합: {}", sum);
}

async fn hello_world() {
    println!("future 안에서 실행");
}

async fn calc_sum(start: i32, end: i32) -> i32 {
    let mut sum = 0;

    for i in start..=end {
        sum += i;
    }

    sum
}

async fn calc() -> i32 {
    let sum1_50 = calc_sum(1, 50).await;
    let sum51_100 = calc_sum(51, 100).await;
    let ret = sum1_50 + sum51_100;

    ret
}

async fn sleep_10sec() {
    for i in 1..10 {
        println!(".");
        time::sleep(Duration::from_millis(1000)).await;
    }
}

async fn calc_sum2(start: i32, end: i32) -> i32 {
    let mut sum = 0;
    for i in start..=end {
        println!("{} ", i);
        sum += i;
    }
    
    sum
}

async fn calc2() -> i32 {
    let f1 = sleep_10sec();
    let f2 = calc_sum2(1, 10);
    let (_, sum) = tokio::join!(f1, f2);
    sum
}

fn inc_counter() {
    let mut num = counter.lock().unwrap();
    *num = *num + 1;
}

#[tokio::test]
async fn async_programming() {
    let future = hello_world();
    println!("main 함수에서 실행");

    future.await;
    println!("future 종료 이후 실행");
}

#[tokio::test]
async fn async_calc() {
    let future = calc_sum(1, 100);
    let sum = future.await;
    println!("1부터 100까지의 합: {}", sum);
}

#[tokio::test]
async fn async_calc_sametime() {
    let res = calc().await;   

    println!("result value is {}", res);
}

#[tokio::test]
async fn async_problem_canbe_occured() {
    let future = calc2();
    let sum = future.await;
    println!("1부터 10까지의 합: {}", sum);
}

#[test]
fn test_eventloop() {
    println!("아무 내용이나 입력하세요. quit를 입력하면 종료됩니다.");

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let input = buf.trim();
        if input == "quit" {
            break;
        }

        println!("입력: {}", input);
    }
}

static counter: Mutex<i32> = Mutex::new(0);

#[test]
fn test_mutex() {
    let mut thread_vec = vec![];

    for _ in 0..100 {
        let th = thread::spawn(inc_counter);
        thread_vec.push(th);
    }

    for th in thread_vec {
        th.join().unwrap();
    }

    println!("결과: {}", *counter.lock().unwrap());
}