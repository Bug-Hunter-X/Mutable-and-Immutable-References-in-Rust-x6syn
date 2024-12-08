fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modify x through y
    }
    println!("x: {}", x); // Output: x: 10

    //The below code will not compile since y is out of scope, preventing concurrent mutable references
    // let z = &mut x;
    // *z = 15; 
} 