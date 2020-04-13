fn main() {
    test1();
    test2();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test1(){
    let mut u1: User = User {
        username: String::from("username"),
        email: String::from("ben.zhai@grapecity.com"),
        active: true,
        sign_in_count: 1,
    };
    u1.username = String::from("another_username");
    println!("{:#?}", u1);
}

#[derive(Debug)]
struct Rect {
    height: i32,
    width: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.width
    }
    fn int(width: i32, height: i32) -> Rect {
        Rect { width,height }
    }
}

fn test2(){
    let rect = Rect::int(30,30);
    println!("{}", (&rect).area());
    println!("{}", rect.area());
    let child = Rect::int(20, 20);
    println!("{}", rect.can_hold(&child));
}