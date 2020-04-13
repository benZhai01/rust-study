fn main() {
    println!("Hello, world!");
    A::B::function_b1();
    let stru = A::C::CStru::init();
    stru.method();
}

mod A {
    pub mod B {
        pub fn function_b1() {
            println!("functionB1");
        }

        fn function_b2() {
            println!("functionB2");
        }
    }

    pub mod C {

        pub struct CStru {

        }

        impl CStru {
            pub fn method(&self){
                println!("CStru method");
            }

            pub fn init() -> CStru {
                super::B::function_b1(); //crate::A::B::function_b1();
                CStru {}
            }
        }
    }
}