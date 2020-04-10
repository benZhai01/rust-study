fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
    test10();
}

fn test1(){
    let mut s = String::from("hello"); // mut is must
    s.push_str(", world");
    println!("{}", s);
}

fn test2(){
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1); error, s1 is moved to s2, s1 is droped
    println!("{}", s2);
}

fn test3(){
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 : {}, s2: {}", s1, s2);
}

fn test4(){
    let s = String::from("hello");
    takes_ownership(s);
    //takes_ownership(s); error.=, s is moved

    let s1 = String:: from("hello");
    println!("{}", s1);
    println!("{}", s1); // why is s not moved in this case ????? println does not take the ownership??
}

fn takes_ownership(s: String){
    println!("{}", s);
}

fn test5(){
    let s = String::from("hello");
    let s = takes_and_gives_back(s);
    println!("{}", s);
}

fn takes_and_gives_back(s: String) -> String{
    String::from("world")
}

fn test6(){
    let s = String::from("hello");

    let (s1, len) = calc_len(s);

    println!("string : {}, len: {}", s1, len);
}

fn calc_len(str: String) -> (String, usize){
    //(str, str.len()) error, second str is moved
    let len = str.len();
    (str, len)
}

fn test7(){
    let s = String::from("hello");

    let len = calc_len_override(&s);

    println!("string is {}, len is {}", s, len);
}

fn calc_len_override(str: &String) -> usize {
    str.len()
}

fn test8(){
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn test9(){
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    //let r3 = &mut s; error
    println!("r1 : {}, r2: {}", r1, r2);

    println!("r1 : {}", r1);
    let r3 = &mut s;
    println!("r3 : {}", r3);

}

fn test10(){
    let s: String = String::from("hello");

    let fw = first_word(&s);

    // s.clear(); err
 
    println!("{}", fw);

    let s2: &str = "hello";
    let fw = first_word2(s2);

    println!("{}", fw);

    let fw = first_word2(&s[..]);

    println!("{}", fw);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if b == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word2(s: &str) -> &str {
    let bytes = &s.as_bytes();
    for(i, &b) in bytes.iter().enumerate() {
        if b == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn test11(){
    let a: [i32; 5] = [1,2,3,4,5];
    let slice: &[i32] = &a[..2];

    //let mut b = [1,2,3,4];
    //let slice = b[1..2]; error must have a specified size on compilation time.
}

