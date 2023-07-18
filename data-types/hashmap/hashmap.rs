use std::collections::HashMap;

fn main(){
    let blue=String::from("Blue");
    let red=String::from("Red");

    let mut map:HashMap<String,i32>=HashMap::new();
    map.insert(blue,10);
    map.insert(red,20);

    let blue=String::from("Blue");
    println!("{:#?}",map.get(&blue));

    for(key,value) in map{
        println!("Key: {}",key);
        println!("Value: {}",value);
    }

}