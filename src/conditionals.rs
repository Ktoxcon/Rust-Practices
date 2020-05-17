    let number:i32 = 32;
    let hexa:i32 = 0x32;
    
    if number == 32 {
        println!("{}",number);
    }if true == (1 != 0) {
        println!("True");
    }if hexa == 50 {
        println!("{}",hexa);
    }

    let divider:i32 = 0;
    let condition = divider != 0;
    let dividing:i32 = 20;
    let its_posible = if condition {"Its posible"} else {"Division by 0 "};
    println!("{}",its_posible);
    let result = dividing / divider;
    println!("{}",result);