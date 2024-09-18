fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    
    // Uncommenting the next line will result in a compilation error
     x += 6; // error: cannot assign twice to immutable variable `x`
     println!("The value of x is: {}", x);

}
