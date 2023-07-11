fn main(){
    let _x:i64 = 89;
    println!("{}", i16::MAX);

    let y:u16 =251_u16 + 9; //here _u16 represents the data type


    let y1:i8= 125;
    let y2:i8= 9;
    let z:i16 = i16::from(y1) + i16::from(y2); // to convert data type while calculating we use :: operator
    println!("{} {}", y,z);
}