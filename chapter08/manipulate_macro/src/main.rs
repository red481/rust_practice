macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

macro_rules! create_accessors {
    ($name:ident, $type:ty, $setter:ident) => {
        fn $name(&self) -> &$type {
            &self.$name
        }

        fn $setter(&mut self, value: $type) {
            self.$name = value;
        }
    };
}

macro_rules! say_hello {
    ($name: expr) => {
        println!("안녕! {}!", $name);
    };
}

macro_rules! example1 {
    ($x:expr) => ( println!("Value: {}", $x) );
}

macro_rules! example2 {
    [$x:expr] => { println!("Value: {}", $x) };
}

macro_rules! example3 {
    {$x:expr} => { println!("Value: {}", $x) };
}

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("함수: {:?}()", stringify!($func_name));
        }
    };
}

create_function!(ident_func);

struct Person {
    name: String,
    age: u32,
}

impl Person {
    create_accessors!(name, String, set_name);
    create_accessors!(age, u32, set_age);
}


#[test]
fn test_create_setter() {
    let mut person = Person { name: "루나".to_string(), age: 10 };
    person.set_name("하이".to_string());
    person.set_age(8);

    println!("이름: {} 나이: {}", person.name, person.age)
}


#[test]
fn test_create_func() {
    ident_func();
}

#[test]
fn test_hello_macro() {
    say_hello!("러스트");
}

#[test]
fn test_add_macro() {
    let sum = add!(1, 2);

    println!("{} + {} = {}", 1, 2, sum);
}

/*
    반복
 */

macro_rules! multi_var {
    ($($var:ident: $type:ty), *) => {
        $(
            let mut $var: $type = Default::default();
        )*
    };
}

macro_rules! add {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

macro_rules! multiply {
    ($x:expr, $y:expr) => {
        $x * $y
    };
}

macro_rules! compute {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        multiply!(add!($a, $b), add!($c, $d))
    };
}

macro_rules! S {
    ($e:expr) =>{ String::from($e) };
}

#[test]
fn test_macro_debugging() {
    let world = S!("World");
    println!("Hello, {}!", world);
}

#[test]
fn test_call_other_macro() {
    let result = compute!(1, 2, 1, 2);
    println!("(1+2)x(1+2)={}", result);
}

#[test]
fn test_iteration() {
    multi_var!(x: u32, y: f64, z: String);
}
