fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y = 10; // Modify x through y
    println!("x: {}", x); // Output: x: 10
    println!("y: {}", *y); // Output: y: 10
    println!("z: {}", *z); // Output: z: 10

    // Trying to modify x through z will result in a compile-time error
    //*z = 15; // This line will cause a compiler error
}