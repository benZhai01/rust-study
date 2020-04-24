fn main() {
    println!("Hello, world!");
    let l = longest("x: &'a str", "y: &'a str");
    println!("{}", l);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}