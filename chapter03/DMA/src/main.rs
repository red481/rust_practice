use std::cell::RefCell;
use std::rc::{ Rc, Weak };

fn main() {

/*     let mut ppp1 = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None),
    });

    let mut ppp2 = Rc::new(Person {
        name: String::from("Dust"),
        age: 20,
        next: RefCell::new(None),
    });

    let mut next = ppp1.next.borrow_mut();
    *next = Some(ppp2.clone());
 */
/* 
    let mut head = Rc::new(Person {
        name: String::from("Luna"),
        age: 30,
        next: RefCell::new(None),
    });

    let tail = push_back(head.clone(), String::from("Rust"), 20);
    let tail = push_back(tail.clone(), String::from("Dust"), 10);

    let mut current = head.clone();
    loop {
        print!("{} -> ", current.name);
        let t = current.clone();
        current = match &(*(t.next.borrow_mut())) {
            Some(p) => p,
            None => break,
        }.clone();
    } */

   let mut p1 = Rc::new(Person {
        id: 1,
        next: RefCell::new(None),
   });

   let mut p2 = Rc::new(Person {
        id: 2,
        next: RefCell::new(None),
   });

   let mut p3 = Rc::new(Person {
        id: 3,
        next: RefCell::new(None),
   });

   let mut next = p1.next.borrow_mut();
   *next = Some(Rc::downgrade(&p2));

   let mut next = p2.next.borrow_mut();
   *next = Some(Rc::downgrade(&p1));

   println!("p1 RefCount: {} p2: RefCount: {}", Rc::strong_count(&p1), Rc::strong_count(&p2));

   println!("p1 RefCount: {} p2: RefCount: {}",
    Rc::strong_count(&p1), Rc::strong_count(&p2));


}

/* fn push_back(tail: Rc<Person>, name: String, age: i32) -> Rc<Person> {
    let p = Rc::new(Person {
        name: name,
        age: age,
        next: RefCell::new(None),
    });

    let mut next = tail.next.borrow_mut();
    *next = Some(p.clone());

    p
} */
/* fn push_front(head: Rc<Person>, name: String, age: i32) -> Rc<Person> {

    let p = Rc::new(Person {
        name: name,
        age: age,
        next: Some(head.clone()),
    });

    p.clone()
} */

struct Person {
    id : i32,
    next: RefCell<Option<Weak<Person>>>,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("p{} Drop!", self.id);
    }
}

