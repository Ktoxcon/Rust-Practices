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

    // UNSIGNED INTEGER
    let eight:u8 = 255;
    let six_teen:u16 = 1000;
    let tirty_two:u32 = 32637;
    let sixty_four:u64 = 9546678;
    let one_two_eight:u128 = 12345678908934567;
    //throws an error, an unsigned int cant be negative
    //let error:u16 = -3;
    println!("{},{},{},{},{}",eight,six_teen,tirty_two,sixty_four,one_two_eight);


    //SIGNED INT
    let mut signed_int:i16 = -30;
    println!("{}",signed_int);
    signed_int = 90;
    println!("{}",signed_int);
    let signed_int = signed_int -34;
    println!("{}",signed_int);
}
