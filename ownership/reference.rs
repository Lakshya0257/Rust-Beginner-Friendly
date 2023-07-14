fn main() {
    let mut x = 5; // Declare a mutable variable x and assign it the value 5

    let y = &mut x; // Mutable borrow of x using a mutable reference

    *y += 1; // Modify the value through the mutable reference

    println!("x: {}", x); // Prints "x: 6"
}
