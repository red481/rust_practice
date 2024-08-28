use std::collections::HashMap;
use std::rc::Rc;

trait Control {
    fn draw(&self) -> String;
}

struct Button {
    name: String,
    decorators: Vec<Box<dyn Control>>,
}

impl Control for Button {
    fn draw(&self) -> String {
        let mut txt = format!("{}", self.name);

        for decorator in self.decorators.iter() {
            txt = format!("{} and {}", txt, decorator.draw());
        }

        txt
    }
}

impl Button {
    fn new(name: String) -> Button {
        Button {
            name: name,
            decorators: Vec::new(),
        }
    }

    fn add_decorator(& mut self, decorator: Box<dyn Control>) {
        self.decorators.push(decorator);
    }
}

struct Deco {
    name: String
}

impl Control for Deco {
    fn draw(&self) -> String {
        format!("{}", self.name)
    }
}

impl Deco {
    fn new(name: String) -> Box<Deco> {
        Box::new(
            Deco {
                name: name
            }
        )
    }
}

#[test]
fn test_decorator() {
    let mut button = Button::new(String::from("참깨빵"));
    button.add_decorator(Deco::new(String::from("순쇠고기")));
    button.add_decorator(Deco::new(String::from("패티두장")));

    println!("{}", button.draw());
}


/*
 * 플라이웨이트 패턴 구현
 */

trait Flyweight {
    fn get_name(&self) -> String;
}

struct ConcreteFlyweight {
    name: String,
}

impl Flyweight for ConcreteFlyweight {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

struct FlyweightFactory {
    flyweights: HashMap<String, Rc<Box<dyn Flyweight>>>,
}

impl FlyweightFactory {
    fn new() -> FlyweightFactory {
        FlyweightFactory {
            flyweights: HashMap::new(),
        }
    }

    fn get_flyweight(&mut self, name: String) -> Rc<Box<dyn Flyweight>> {
        if let Some(instance) = self.flyweights.get(&name) {
            return instance.clone();
        }

        let instance = Box::new(ConcreteFlyweight {
            name: name.clone(),
        });

        let instance = Rc::new(instance as Box<dyn Flyweight>);
        self.flyweights.insert(name.clone(), instance.clone());

        instance.clone()
    }
}

#[test]
fn test_flyweight() {
    let mut factory = FlyweightFactory::new();

    let flyweight1 = factory.get_flyweight(String::from("위키"));
    let flyweight2 = factory.get_flyweight(String::from("북스"));
    let flyweight3 = factory.get_flyweight(String::from("북스"));

    println!("{}", flyweight1.get_name());
    println!("{}", flyweight2.get_name());
    println!("{}", flyweight3.get_name());
}

/*
 * 옵저버 패턴
 */

#[derive(PartialEq)]
struct Listener {}

impl Listener {
    fn on_event(&self, data: &str) {
        println!("Event 발생: {}", data);
    }
}

struct Subject {
    observers: Vec<Rc<Listener>>,
}

impl Subject {
    fn subscribe(&mut self, observer: Rc<Listener>) {
        self.observers.push(observer);
    }

    fn unsubscribe(&mut self, observer: Rc<Listener>) {
        if let Some(index) = self.observers.iter().position(|o| *o == observer) {
            self.observers.remove(index);
        }
    }

    fn notify(&self, data: &str) {
        for observer in &self.observers {
            observer.on_event(data);
        }
    }
}

#[test]
fn test_observer_pattern() {
    let mut subject = Subject {
        observers: Vec::new(),
    };

    let observer1 = Rc::new(Listener {});
    let observer2 = Rc::new(Listener {});

    subject.subscribe(observer1.clone());
    subject.subscribe(observer2.clone());

    subject.notify("이벤트 #1");

    subject.unsubscribe(observer1.clone());

    subject.notify("이벤트 #2");

    subject.unsubscribe(observer2.clone());
}

/*
 * 전략 패턴
 */

trait Render {
    fn render(&self, title: String, body: String);
}

struct HtmlRenderer {}

impl Render for HtmlRenderer {
    fn render(&self, title: String, body: String) {
        println!("<html><title>{}</title><body>{}</body></html>", title, body);
    }
}

struct MarkdownRenderer;

impl Render for MarkdownRenderer {
    fn render(&self, title: String, body: String) {
        println!("# {}\n{}", title, body);
    }
}

struct Page<T: Render> {
    renderer: T,
}

impl<T: Render> Page<T> {
    fn new(renderer: T) -> Page<T> {
        Page { renderer }
    }

    fn renderer(&self, title: String, body: String) {
        self.renderer.render(title, body);
    }
}

#[test]
fn test_strategy_pattern() {
    let html = Page::new(HtmlRenderer {});
    html.renderer.render(String::from("제목"), String::from("내용"));

    let markdown = Page::new(MarkdownRenderer {});
    markdown.renderer.render(String::from("제목"), String::from("내용"));
}

/*
 * 상태 패턴
 */

trait State {
    fn on_state_changed(self: Box<Self>) -> Box<dyn State>;
}

struct Start;
impl State for Start {
    fn on_state_changed(self: Box<Start>) -> Box<dyn State> {
        println!("현재 상태는 [Start], 다음 상태는 [Running]");
        Box::new(Running {})
    }
}

struct Running;
impl State for Running {
    fn on_state_changed(self: Box<Running>) -> Box<dyn State> {
        println!("현재 상태는 [Running], 다음 상태는 [Stop]");
        Box::new(Stop {})
    }
}

struct Stop;
impl State for Stop {
    fn on_state_changed(self: Box<Stop>) -> Box<dyn State> {
        println!("현재 상태는 [Stop], 다음 상태는 [없음]");
        self
    }
}

struct Boot {
    state: Option<Box<dyn State>>,
}

impl Boot {
    fn new() -> Boot {
        Boot {
            state: Some(Box::new(Stop {})),
        }
    }

    fn boot(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(Box::new(Start {}))
        }
    }

    fn next(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.on_state_changed())
        }
    }
}

#[test]
fn test_state_pattern() {
    let mut post = Boot::new();
    post.boot();
    post.next();
    post.next();
    post.next();
}