trait Pizza {
    fn eat(&self);
}

trait Hello {
    fn hello_msg(&self) -> String;
}

trait PizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza>;
}

enum PizzaType {   
    Bulgogi,
    Hawaiian,
}

struct ConcretePizzaFactory {}
struct BulgogiPizza {}
struct Hawaiianpizza {}

impl Pizza for BulgogiPizza {
    fn eat(&self) {
        println!("불고기 냠냠");
    }
}

impl Pizza for Hawaiianpizza {
    fn eat(&self) {
        println!("파인애플 냠냠");
    }
}

impl PizzaFactory for ConcretePizzaFactory {
    fn create(pizza_type: PizzaType) -> Box<dyn Pizza> {
        match pizza_type {
            PizzaType::Bulgogi => Box::new(BulgogiPizza{}),
            PizzaType::Hawaiian => Box::new(Hawaiianpizza{}),
        }
    }
}


#[test]
fn test_factoy_method() {
    let bulgogi = ConcretePizzaFactory::create(PizzaType::Bulgogi);
    let hawaiian = ConcretePizzaFactory::create(PizzaType::Hawaiian);

    bulgogi.eat();
    hawaiian.eat();

}

// #[test]
// fn test_capsulation() {
//     let student = Student::new(1, String::from("luna"), String::from("luna@email.com"));
//     println!("이름: {}", student.get_name());
// }

fn say_hello(say: &dyn Hello) {
    println!("{}", say.hello_msg());
}

struct Student {}

impl Hello for Student {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요! 선생님,")
    }
}

struct Teacher {}

impl Hello for Teacher {
    fn hello_msg(&self) -> String {
        String::from("안녕하세요. 오늘 수업은...")
    }
}

#[test]
fn test_trait_instance() {
    let student = Student {};
    let teacher = Teacher {};

    say_hello(&student);
    say_hello(&teacher);
}

trait Pointable {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

struct Point {
    x: i32,
    y: i32,
}

impl Pointable for Point {
  
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }
}

struct ColorPoint {
    color: String,
    point: Point,
}

impl ColorPoint {
    fn new(color: String, x: i32, y: i32) -> ColorPoint {
        ColorPoint {
            color: color,
            point: Point {
                x: x,
                y: y
            }
        }
    }

    fn color(&self) -> &String {
        &self.color
    }
}

impl Pointable for ColorPoint {
    fn x(&self) -> i32 {
        self.point.x
    }

    fn y(&self) -> i32 {
        self.point.y
    }
}

#[test]
fn test_subclass() {
    let pt = ColorPoint::new(String::from("red"), 1, 2);
    print_pointable(&pt);
}

struct Burger {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl Burger {
    fn to_string(&self) -> String {
        let mut txt = format!("{} 위에 순 쇠고기 패티 {}장 {} 소스 ",
        self.bun, self.patties, self.sauce);

        for ex in self.extra.iter() {
            txt = format!("{} {} ", txt, ex);
        }

        txt
    }
}

struct BurgerBuilder {
    bun: String,
    patties: i32,
    sauce: String,
    extra: Vec<String>,
}

impl BurgerBuilder {
    fn new() -> BurgerBuilder {
        BurgerBuilder {
            bun: String::from(""),
            patties: 0,
            sauce: String::from(""),
            extra: Vec::<String>::new()
        }
    }

    fn bun(mut self, bun: String) -> BurgerBuilder {
        self.bun = bun;
        self
    }

    fn patties(mut self, patties: i32) -> BurgerBuilder {
        self.patties = patties;
        self
    }

    fn sauce(mut self, sauce: String) -> BurgerBuilder {
        self.sauce = sauce;
        self
    }

    fn add_extra(mut self, val: String) -> BurgerBuilder {
        self.extra.push(val);
        self
    }

    fn build(self) -> Burger {
        Burger {
            bun: self.bun,
            patties: self.patties,
            sauce: self.sauce,
            extra: self.extra,
        }
    }
}

fn print_pointable(pointable: &dyn Pointable) {
    println!("x: {} y: {}", pointable.x(), pointable.y());
}

#[test]
fn test_builder() {
    let burger = BurgerBuilder::new()
    .bun(String::from("참깨빵"))
    .patties(2)
    .sauce(String::from("특별한"))
    .add_extra(String::from("양상추"))
    .build();

    println!("{}", burger.to_string());
}

struct Adaptee {}

impl Adaptee {
    fn new() -> Adaptee {
        Adaptee {}
    }

    fn vendor_specific_api(&self) {
        println!("벤더가 정의한 API");
    }
}

struct Adapter {}

impl Adapter {
    fn new() -> Adapter {
        Adapter {}
    }

    fn call_api(&self) {
        Adaptee::new().vendor_specific_api();
    }
}

#[test]
fn test_adapter() {
    let adapter = Adapter::new();
    adapter.call_api();
}

trait Control {
    fn draw(&self) -> String;
}

struct Button {
    name: String,
}

struct Panel {
    name: String,
    controls: Vec<Box<dyn Control>>,
}

impl Control for Button {
    fn draw(&self) -> String {
        format!("Button - {}", self.name)
    }
}

impl Control for Panel {
    fn draw(&self) -> String {
        let mut txt = format!("Panel - {}", self.name);
        for control in self.controls.iter() {
            txt = format!("{}\n\t{}", txt, control.draw());
        }
        txt
    }
}

impl Panel {
    fn new(name: String) -> Box<Panel> {
        Box::new(Panel {
            name: name,
            controls: Vec::new()
        })
    }

    fn add_control(& mut self, control: Box<dyn Control>) {
        self.controls.push(control);
    }
}

impl Button {
    fn new(name: String) -> Box<Button> {
        Box::new(Button {
            name: name
        })
    }
}

#[test]
fn test_composite() {
    let mut root = Panel::new(String::from("root"));
    root.add_control(Button::new(String::from("Button #1")));

    let mut panel = Panel::new(String::from("panel #1"));
    panel.add_control(Button::new(String::from("button #2")));
    root.add_control(panel);

    println!("{}", root.draw());
}


fn main() {

}