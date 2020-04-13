fn main() {
   test1();
   test2();
   test3();
   test4();
   test5();
}

#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IPAddr {
    kind: IpAddKind,
    addr: String,
}

fn test1(){
    let home: IPAddr = IPAddr {kind: IpAddKind::V4, addr: String::from("10.32.1.1")};
    let loopback: IPAddr = IPAddr {kind: IpAddKind::V6, addr: String::from("::1")};
    println!("home: {:#?}", home);
    println!("home: {:#?}", loopback);
}

#[derive(Debug)]
enum IPAddrE {
    V4(String),
    V6(String),
}

fn test2() {
    let home: IPAddrE = IPAddrE::V4(String::from("10.32.1.1"));
    let loopback : IPAddrE = IPAddrE::V6(String::from("::1"));
    println!("{:#?}", home);
    println!("{:#?}", loopback);
    
}

fn test3(){
    let mut x: Option<IPAddrE> = None;
    x = Some(IPAddrE::V4(String::from("10.32.1.1")));
}

fn test4(){
    fn inner(kind: IpAddKind) -> i32 {
        match kind {
            IpAddKind::V4 => 1,
            IpAddKind::V6 => {
                println!("IP V6");
                2
            }
        }
    }
    inner(IpAddKind::V4);
    inner(IpAddKind::V6);

    fn inner2(kind: IPAddrE) -> i32 {
        match kind {
            IPAddrE::V4(addr) => {
                println!("{}", addr);
                1
            },
            IPAddrE::V6(addr) => {
                println!("{}", addr);
                2
            }
        }
    }
    inner2(IPAddrE::V4(String::from("10.32.1.1")));
    inner2(IPAddrE::V6(String::from("::1")));
}

fn test5(){
    fn inner1(kind: IpAddKind) -> i32{
        let mut c1 = 0;
        match kind {
            IpAddKind::V4 => println!("V4"),
            _ => c1 += 1,
        }
        c1
    }
    inner1(IpAddKind::V4);

    fn inner2(kind: IpAddKind) -> i32{
        let mut c1 = 0;
        if let IpAddKind::V4 = kind {
            println!("V4");
        }else{
            c1 += 1;
        }
        c1
    }
    inner2(IpAddKind::V4);
}