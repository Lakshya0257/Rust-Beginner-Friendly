fn main(){
    let x: i16 = 5;
    let y: i16 = 1;
    let z: i32 = sum(i32::from(x), y as i32);
    println!("{}", z);
}

fn sum(x: i32, y: i32) -> i32 {
    x+y
}