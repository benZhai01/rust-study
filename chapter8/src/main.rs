fn main() {
    test1();
    test2();
    test3();
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