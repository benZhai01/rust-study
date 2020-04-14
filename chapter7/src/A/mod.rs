
pub mod C;
mod B;

pub fn A_fun(){
    B::function_b1();
    let stru = C::CStru::init();
    stru.method();
}