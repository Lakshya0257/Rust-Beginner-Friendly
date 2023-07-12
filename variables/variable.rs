use std::mem::size_of_val;


fn main(){
    println!("---------Main function----------");
    let x: i32=5;
    assert_eq!(x,5);  //checking the value, if not true then it will terminate
    println!("Success");
    println!();

    //mutable declaration
    mutable();

    //shadowing the variable
    shadowing();

    //range function
    println!("---------Scope Declaration----------");
    {
        let y:i32=50;
        let mut x:i32 =x; //we can redeclare variable inside scope with same name
        x=x+30; //all the computation of that varibale will only be in that particular scope
        println!("The value of y and x will be {} {}",y,x);
    }
    println!("{}",x); //prints 5
    println!();


    //other data types
    char();
    bool();
}

fn mutable(){
    println!("---------Mutable function----------");
    let mut y: i32=10;
    y=y+20;
    println!("{}",y); //we need to create block for varible printing
    println!();
}

fn shadowing(){
    println!("---------Shadowing the variable----------");
    let mut x:i32=20;
    x=x+9;
    let x:i32=10;
    println!("After shadowing, value of x will be {}",x);
    println!();
}


fn char(){
    println!("---------Character----------");
    let c:char='a';
    println!("{}",size_of_val(&c));
    println!();
}


fn bool(){
    println!("---------Boolean----------");
    let b:bool=true;
    if b{
        println!("True");
    }
    println!();
}