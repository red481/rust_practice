use std::io;

fn main() {
    println!("학번을 입력해주세요.");
    let mut id = String::new();
    let _ = io::stdin().read_line(&mut id);
    let id: i32 = id.trim().parse().unwrap();

    println!("이름을 입력해주세요.");
    let mut name = String::new();
    let _ = io::stdin().read_line(&mut name);
    let name = name.trim().to_string();

    println!("이메이을 입력해주세요.");
    let mut email = String::new();
    let _ = io::stdin().read_line(&mut email);
    let email = email.trim().to_string();

    let stu = create_student(id, name, email);
    println!("stu={:?}", stu);
}

fn create_student(id: i32, name: String, email: String) -> Student {
    Student {
        id: id,
        name: name,
        email: email,
    }
}

#[derive(Debug)]
struct Student {
    id: i32,
    name: String,
    email: String,
}

struct Score {
    score : i32,
}

impl Score {
    fn get_grade(&self) -> String {
        if self.score > 90 {
            String::from("A")
        } else if self.score > 80 {
            String::from("B")
        } else {
            String::from("C")
        }
    }

    fn from(score: i32) -> Score {
        Score { score: score }
    }
}

#[test]
fn test_get_grade() {
    let score = Score {
        score: 100,
    };
    assert_eq!(score.get_grade(), "A");

    let score = Score {
        score: 90,
    };
    assert_eq!(score.get_grade(), "B");

    let score = Score {
        score: 80
    };
    assert_eq!(score.get_grade(), "C");

    assert_eq!(Score::from(100).get_grade(), "A");
    assert_eq!(Score::from(90).get_grade(), "B");
}