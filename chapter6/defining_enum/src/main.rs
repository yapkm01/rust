#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message is: {self:?}");
    }
}

struct FullName  {
    first: String,
    middle: Option<String>,
    last: String,
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    println!("value of home is: {home:?}");
    println!("value of loopback is: {loopback:?}");

    let m = Message::Write(String::from("hello"));
    m.call();
}