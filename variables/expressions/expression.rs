fn main() {
    let x: i64 = 40;
    let z: i32 = 40;
    match_function(x,z as i64);
    assert_eq!(x,z as i64);
    let y: i32={
        let x: i32= x as i32;

        //if we wanted to assign a value to a variable after computation 
        //then we use computation inside the scope and then without semicolon 
        //we return/assign the value
        
        x+2
    };
    println!("x={}",x);
    println!("y={}",y);
}

fn match_function(a:i64, b:i64){
    match a{
       40 =>{
        println!("1st");
       }
       _ => {
        println!("2nd");
       }
    }
}