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

impl Rectangle{
    fn createSquare(size : u32) -> Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
}

fn main(){
    let dem:Rectangle = Rectangle{
        width: 34,
        height: 25
    };

    let sqaure:Rectangle = Rectangle::createSquare(25);

    let area: u32 = Rectangle::area(&dem); //calling the implemented function
                 //OR
    let area1: u32 = dem.area(); //calling the implemented function

    assert_eq!(area,area1);

    println!("{}",area);
    println!("{:?}",sqaure);
}

