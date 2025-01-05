fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modifying x through y
    println!("x: {}", x); // Prints 10
    println!("z: {}", *z); //Prints 10, this may be unexpected as z is immutable
}