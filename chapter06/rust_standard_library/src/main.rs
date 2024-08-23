use std::process::Command;
use std::process;
use std::env;

impl From<(i32, i32)> for Point {
    fn from(tuple: (i32, i32)) -> Self {
        Point { x: tuple.0, y: tuple.1 }
    }
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Book 객체 해제: {}", self.title);
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    cloned: bool,
}

impl AsMut<String> for Person {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

impl AsRef<str> for Person {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
            cloned: true
        }
    }
}

fn add_points(p1: Point, p2: Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

#[test]
fn test_copy_trait() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };

    let result = add_points(a, b);

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", result);
}

#[test]
fn test_clone_trait() {
    let mut person1 = Person {
        name: String::from("루나"),
        age: 10,
        cloned: false
    };

    let mut person2 = person1.clone();
    person2.name = String::from("지져스");

    println!("{:?}", person1);
    println!("{:?}", person2);
    
}

#[test]
fn test_drop_trait() {
    {
        let book = Book { title: String::from("러스트") };
    }
}

#[test]
fn test_from_trait() {
    let tuple = (1, 2);

    let pt: Point = Point::from(tuple);

    println!("Point::from = {:?}", pt);

    let pt: Point = tuple.into();

    println!("tuple.into = {:?}", pt);
}

fn greet_person<P: AsRef<str>>(person: P) {
    println!("안녕! {}!", person.as_ref());
}

#[test]
fn test_asref() {
    let person = Person { name: String::from("루나"), age: 30, cloned: true };

    greet_person(person);

    greet_person(String::from("러스트"));
    greet_person("하이!");
}

fn name_change<P: AsMut<String>>(person: &mut P, new_name: &str) {
    let name = person.as_mut();
    name.clear();
    name.push_str(new_name);
}

#[test]
fn test_asmut() {
    let mut person = Person {
        name: String::from("루나"),
        age: 10,
        cloned: false,
    };

    println!("변경 전: {}", person.name);
    name_change(&mut person, "러스트");
    println!("변경 후: {}", person.name);
}

#[test]
fn test_env_configured() {
    for (index, argument) in env::args().enumerate() {
        println!("인자 #{}: {}", index, argument);
    }
}

#[test]
fn test_env_read() {
    env::set_var("my_env", "my_value");

    match env::var("my_env") {
        Ok(value) => println!("my_env = {}", value),
        Err(e) => println!("my_env 읽기 오류: {}", e)
    }

    env::remove_var("my_env");
}

#[test]
fn test_configure_directorypath() {
    match env::current_dir() {
        Ok(path) => println!("현재 경로: {:?}", path),
        Err(e) => println!("현재 경로 획득 실패: {}", e),
    }

    match env::temp_dir().to_str() {
        Some(path) => println!("임시 경로: {}", path),
        None => println!("임시 경로 확인 불가")
    }
}

#[test]
fn test_process() {
    let echo = Command::new("echo")
        .arg("echo 실행")
        .output()
        .expect("echo 실행 실패");

    let ret = String::from_utf8_lossy(&echo.stdout);

    println!("결과: {}", ret);
}

#[test]
fn test_pid() {
    let pid = process::id();
    println!("Process ID: {}", pid);
}

fn main() {

}
