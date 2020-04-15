use std::collections::HashMap;

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
}

fn test1(){
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(4);
    v.push(2);
    v.push(3);

    v.sort();

    let mut v1 = vec![1,2,3];

    v1.push(4);
}

fn test2(){
    let mut v = vec![1,2,3,4,5];

    let mut _third = &v[2];
    //v.push(6); error
    if *_third != 2 {
        println!("_third is {}", _third);
    }
    println!("_third is {}", _third);
    _third = &(12);
    println!("_third is {}", _third);
    v.push(6);
    
    match &v.get(4) {
        Some(i) => println!("{}", i),
        None => println!("Out of index"),
    }
}

fn test3(){
    let mut v = vec![1,2,3];
    
    for i in &mut v {
        *i *= 10;
    }
    
    v[1] = 12;
    for i in &v {
        println!("{}", i);
    }

}

fn test4(){
    let mut s = String::from("foo");
    s.push(' ');
    s.push_str("bar");

    s += " hello";
    print!("{}", s);
}

fn test5(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + &s2 + &s3;
    println!("{}", s);
}

fn test6(){
    let mut hashmap: HashMap<String, i32> = HashMap::new();
    hashmap.insert(String::from("Blue"), 10);
    hashmap.insert(String::from("Yellow"), 30);
    println!("{:#?}", hashmap);

    *hashmap.entry(String::from("Blue")).or_insert(20) *= 2;
    println!("{:#?}", hashmap);
}

fn test7(){
    let words = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in words.split_whitespace() {
        let v: &mut i32 = map.entry(word).or_insert(0);
        *v += 1;
        //*map.entry(word).or_insert(0) += 1;
    }
    println!("{:#?}", map);
}