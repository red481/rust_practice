use sqlite;
use sqlite::State;
use tokio::io::{ stdin, BufReader, Lines };
use tokio::fs::File;
use tokio::io::{ AsyncBufReadExt, AsyncReadExt, AsyncWriteExt };
use serde::{ Serialize, Deserialize };

fn main() {

}
/* 
#[tokio::test]
async fn test_sync() {
    let mut file = File::open("input.txt").await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    let mut file = File::create("output.txt").await.unwrap();
    file.write_all(contents.as_bytes()).unwrap();   
}

#[tokio::test]
async fn test_async() {
    let mut file = File::open("input.txt").await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();

    println!("{}", contents);

    let mut file = File::create("output.txt").await.unwrap();
    file.write_all(contents.as_bytes()).await.unwrap();
} */

#[tokio::test]
async fn test_async_eventloop() {
    let mut reader = BufReader::new(stdin());
    let mut lines = reader.lines();

    loop {
        match lines.next_line().await.unwrap() {
            Some(input) => {
                println!("입력: {}", input);

                if input == "quit" {
                    break;
                }
            },

            None => {
                break;
            },
        };
    }
}

#[tokio::test]
async fn test_buffering() {
    let file = File::open("input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        println!("{}", line);
    }
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn test_serialization() {
    let pt = Point { x: 10, y: 20 };
    let json = serde_json::to_string(&pt).unwrap();
    println!("json: {}", json);

    let pt: Point = serde_json::from_str(&json).unwrap();
    println!("point: [{}, {}]", pt.x, pt.y);
}

#[test]
fn test_sqlite() {
    let connection = sqlite::open(":memory:").unwrap();

    let query = "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('루나', 3);
        INSERT INTO users VALUES ('러스트', 13);
    ";
    connection.execute(query).unwrap();

    let query = "SELECT * FROM users WHERE age > ?";
    let mut statement = connection.prepare(query).unwrap();
    statement.bind((1, 5)).unwrap();

    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name").unwrap());
        println!("age = {}", statement.read::<i64, _>("age").unwrap());        
    }
}