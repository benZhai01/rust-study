fn main() {
    let uint: u32 = 23;
    let c: char = 'A';
    let c: char = '😂';
    let tup: (i32, u32, f32, char) = (1, 2, 2.0, c);
    tup.0; tup.1; tup.2;
    let (x, y , z, u) = tup;
    let arr = [1,2,3,4];
    let arr: [i32; 2] = [1,2];
    //let arr: [i32; 2] = [1,2,4]; error
    let arr = [3; 5]; // equal [3,3,3,3,3]
    another_fun(1, c);
    fn2();
    println!("{}", five(2, 3));
    _if(4);
    _if2();
    _loop();
    _while();
    arr_loop();
}

fn another_fun(x: i32, y: char){
    println!("aaa");
    println!("x: {}, y: {}", x, y);
}

fn fn2(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("{}", y);
}

fn five(x: u32, y: u32) -> u32 {
    //return x * y;
    // or
    x * y
}

fn _if(num: i32){
    if num < 5 { //if(num < 5) is also ok
        println!("small");
    }else{
        println!("big");
    }
}

fn _if2(){
    let n = if false {
        5
    }else{
        // "666" is invalid because the type is different
        666
    };
    println!("{}", n);
}

fn _loop() {
    let mut count = 1;
    let res = loop {
        count += 1;
        if count == 10{
            break count * 2; //can add any return value after break
        }
    };
    println!("{}", res);
}

fn _while(){
    let mut num = 3;
    while num != 0{
        num -= 1;
    }
    println!("{}", num)
}

fn arr_loop(){
    let a = [1,2,3,4,5];
    let mut index = 0;
    while index < a.len() {
        println!("{}", a[index]);
        index +=1 ;
    }

    for el in a.iter(){
        println!("{}", el);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}