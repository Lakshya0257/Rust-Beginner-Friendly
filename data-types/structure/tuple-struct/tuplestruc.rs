#[derive(Debug)]
struct Rectangle{
    height : u32,
    width : u32
}


//assigning an function to a user-defined structure
impl Rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }
}

fn main(){
    let dem:Rectangle = Rectangle{
        width: 34,
        height: 25
    };
    let area: u32 = dem.area(); //calling the implemented function
    println!("{}",area);
    println!("{:?}",dem);
}

