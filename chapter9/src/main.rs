use std::fs::File;
use std::io::ErrorKind;

fn main() {
    test1();
    test2();
}

fn test1(){
    //panic!("crash");
}

fn test2(){
    let f = File::open("hello.text");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.text"){
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        },
    };
}