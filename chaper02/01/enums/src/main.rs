fn main() {
    let m = Message::Put(String::from("/root/"));
    m.excute();

    let m = Message::List(33);
    m.excute();

    let some_string = Some(String::from("러스트"));
    let none_string: Option<String> = None;

    print_optional(some_string);
    print_optional(none_string);
}

fn print_optional(val: Option<String>) {
    match val {
        Some(val) => println!("{}", val),
        None => println!("None"),
    }
}

enum SchoolKind {
    Elementary(ElementarySchool),
    Middle(MiddleSchool),
    High(HighSchool)
}

enum Message {
    Quit,
    List(i32),
    Put(String),
    Get(i32)
}

impl Message {
    fn excute(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::List(val) => println!("List: {}", val),
            Message::Put(val) => println!("Put: {}", val),
            Message::Get(val) => println!("Get: {}", val)
        }
    }
}

struct ElementarySchool {
    room: String,
}

struct MiddleSchool {
    teacher: String,
}

struct HighSchool {
    id: i32,
}

