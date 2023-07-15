#[derive(Debug)]
enum IpAddressKind{
    V1(String),
    V2(String),
}

impl IpAddressKind{
    fn printIp(&self){
        println!("{:?}",self);
    }
}

// struct IpAddr{
//     kind: IpAddressKind,
//     address: String
// }

fn main(){
    // let localhost: IpAddr = IpAddr{
    //     kind:IpAddressKind::V1,
    //     address: String::from("121.0.1.0")
    // };

    let localhost:IpAddressKind=IpAddressKind::V1(String::from("121.1.0.1")); 
    localhost.printIp();
}