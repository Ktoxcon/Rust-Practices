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
    const KEY_A:char = 'A';
    println!("{}",KEY_A);
    const _POSITION_X:i128 = 0;


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


    // FLOAT POINT VARIABLES
    let float_32:f32 = 3.14;
    println!("{}",float_32);
    let float_64:f64 = 3.1234589;
    println!("{}",float_64);

    // WORKING WITH NUMERIC OPRATIONS
    let sum:i32 = 3 + 2;
    println!("{}",sum);
    let sub:i32 = sum - 2;
    println!("{}",sub);
    let mul:i32 = sum * sub;
    println!("{}",mul);
    let div:f32 = (sum as f32) / (mul as f32);
    println!("{}",div);
    let rem:i32 = sum % sub;
    println!("{}",rem);

    //TUPLES
    let rgb:(i16,i16,i16) = (255,120,0);
    let rgba:(i16,i16,i16,i16) = (255,0,255,10);

    let (r,g,b) = rgb;
    println!("[ {},{},{} ]",r,g,b);
    let (r,g,b,a) = rgba;
    println!("[ {},{},{},{} ]",r,g,b,a);

    let r1:i16 = rgb.0;
    println!("{}",r1);

    let hexa:(char,i16,i16,i16,i16) = ('#',0,0,0,0);
    println!("{}{}{}{}{}",hexa.0,hexa.1,hexa.2,hexa.3,hexa.4);

    //ARRAYS
    let numbers = [1,2,3,4,5];
    println!("{},{},{},{}",numbers[0],numbers[1],numbers[2],numbers[3]);

    let chars:[char;4] = ['A','B','C','D'];
    println!("{}{}{}{}",chars[0],chars[1],chars[2],chars[3]);

    let mut array:[(i32,i32,i32); 2] = [(32,33,43),(45,55,66)];
    array[0].1 = 1;
    println!("{}",array[0].0);

    let hexa:(char,i16,i16,i16,i16) = ('#',0,0,0,0);
    println!("{}{}{}{}{}",hexa.0,hexa.1,hexa.2,hexa.3,hexa.4);

    
    println!("{}",array[0].0);
    println!("{:*^16?}",array[0]);
    println!("Your phone number is:{}{:*>6}",array[0].0,array[1].0);


