extern crate encoding_rs;

use encoding_rs::{EUC_KR, UTF_8};
use std::str;
use std::collections::LinkedList;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

fn main() {

}

#[test]
fn vector_test() {
        let mut v: Vec<i32> = Vec::new();

    for i in 1..10 {
        v.push(i);
    }

    for d in &v {
        print!("{}, ", d);
    }

    let mut v: Vec<i32> = vec![];
    let mut v: Vec<i32> = vec![1, 2, 3];

    let v = vec![1, 2, 3];
    let one = v[0];
    let two = v.get(1);

    println!("One: {:?}, Two: {:?}", one, two);

    let v = vec![1, 2, 3];
    let nine = v.get(9);
    println!(" {:?}", nine);

    let mut v = vec![1, 2, 3];
    v[0] = 2;
    v[1] = 3;
    v[2] = 4;

    println!("{}, {}, {}", v[0], v[1], v[2]);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 1;
    }
}

#[test]
fn test_linkedlist1() {
    let mut list: LinkedList<i32> = LinkedList::new();
    
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    for i in &list {
        print!("{}, ", i);
    }
}

#[test]
fn test_linkedlist2() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

    let idx = 9;
    let mut i = 0;
    let mut target = 0;

    for data in &list {
        if i == idx {
            target = *data;
        }

        i += 1;
    }

    println!("target: {:?}", target);

    let target2 = list.iter().nth(9);
    println!("target2: {:?}", target2);
}

#[test]
fn test_linkedlist3() {
    let mut list: LinkedList<i32> = LinkedList::new();
    for i in 0..10 {
        list.push_back(i);
    }

    for d in list.iter_mut() {
        *d += 10;
    }

    for d in list.iter() {
        print!("{:?}, ", d);
    }
}


#[test]
fn test_hashmap1() {
    let mut books: HashMap<i32, String> = HashMap::new();

    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    for (key, value) in &books {
        println!("Key: {:?}, Valu: {:?}", key, value);
    }
}

#[test]
fn test_hashmap2() {
    let mut books: HashMap<i32, String> = HashMap::new();

    books.insert(10, String::from("Rust"));
    books.insert(20, String::from("Java"));
    books.insert(30, String::from("Python"));

    let rust = match books.get(&10) {
        Some(n) => n,

        None => "no value was stored in here."
    };

    println!("key 10은 {:?}", rust);    
}

#[test]
fn test_hashset1() {
    let mut book: HashSet<String> = HashSet::new();

    book.insert(String::from("Rust"));
    book.insert(String::from("Rust"));
    book.insert(String::from("Rust"));
    book.insert(String::from("Java"));

    for data in &book {
        println!("{:?}", data);
    }

    if book.contains("Python") == false {
        println!("Python이 없습니다.");
    }
}

#[test]
fn test_binaryheap1() {
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();

    heap.push(3);
    heap.push(9);
    heap.push(2);
    heap.push(5);

    while heap.is_empty() == false {
        print!("{:?}, ", heap.pop());
    }
}

#[test]
fn test_string1() {
    let mut eng = String::new();
    eng.push_str("hello!");
    let kor = String::from("안녕하세요!");

    println!("{} {}", eng, kor);
}

#[test]
fn test_string2() {
    let str = String::from("안녕");
    let idx = 123;
    let s = format!("{} {}", str, idx);
    println!("{}", s);
}

#[test]
fn test_string3() {
    let txt = String::from("안녕하세요.");
    for c in txt.chars() {
        print!("{}, ", c);
    }
}

#[test]
fn test_string4() {

    let utf8_string = "안녕하세요";

    let utf8_bytes = utf8_string.as_bytes();
    let (euc_kr_bytes, _, _) = EUC_KR.encode(utf8_string);

    println!("UTF-8: {:?}", utf8_bytes);
    println!("EUC_KR: {:?}", euc_kr_bytes);

    let (utf8_string, _, _) = EUC_KR.decode(&euc_kr_bytes);

    println!("EUC-KR to UTF-8: {}", utf8_string);
}

#[test]
fn test_collection1() {
    let vec = vec![1, 2, 3];
    for item in vec.iter() {
        println!("{}", item);
    }

    println!("{:?}", vec);
}

#[test]
fn test_collection2() {
    let vec = vec![1, 2, 3];
    for item in vec.into_iter() {
        println!("{}", item);
    }

    println!("{:?}", vec);
}