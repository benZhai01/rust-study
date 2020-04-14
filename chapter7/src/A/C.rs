
pub struct CStru {

}

impl CStru {
    pub fn method(&self){
        println!("CStru method");
    }

    pub fn init() -> CStru {
        super::B::function_b1(); 
        CStru {}
    }
}