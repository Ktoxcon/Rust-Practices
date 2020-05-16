fn main() {
    // PRACTICING VARIABLES IN RUST

    // immutable variable, the underscore indicates that this are unused variables
    let _x = 5;
    let _y = 6;

    // this throws an error, because x and y are immutable variables
    // x = 7;

    // adding mut to the beginning of a variable declaration and making it mutable
    let mut mutable = 45;
    println!("{}",mutable);
    mutable = 46;
    println!("{}",mutable);

    // some constants
    const CONSTANT:f32 = 3.14;
    println!("{}",CONSTANT);

    // shadowing variables
    let shadowed = "   ";
    let shadowed = shadowed.len();
    println!("{}",shadowed);

}
