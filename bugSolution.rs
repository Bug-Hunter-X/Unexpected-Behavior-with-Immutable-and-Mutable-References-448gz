fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 10; // Modifying x through y
    } // The scope of mutable reference y ends here

    let z = &x; // z is an immutable reference to x
    println!("x: {}", x); // Prints 10
    println!("z: {}", *z); //Prints 10, this is expected as z is immutable outside of y's scope.

    // To ensure that the value referenced by z remains unchanged even after a mutable reference is used,
    // you need to restrict the scope of the mutable reference.
}