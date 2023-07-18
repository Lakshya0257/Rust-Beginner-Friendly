fn main(){

    //Initializing the vector
    let x:Vec<i32> = vec![1,2,3];
    let mut a:Vec<i32> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    assert_eq!(x,a);

    println!("{:#?}",a);

    //To get the value by index
    let x=&a[1];
    println!("{}",x);

    //Getting value with get function for error handling
    match a.get(20){
        Some(value)=>println!("The value is {}",value),
        None=>println!("There is no specified value on your index")
    }
}