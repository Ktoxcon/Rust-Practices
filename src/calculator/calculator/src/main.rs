use std::io;

fn main() {
    menu()
}

fn menu(){

    println!("{:*>25}","");
    println!("{: >7}{}"," ","CALCULATOR");
    println!("{:*>25}","");
    println!("{: >2}{}","","1. SUM");
    println!("{: >2}{}","","2. SUB");
    println!("{: >2}{}","","3. MUL");
    println!("{: >2}{}","","4. DIV");
    println!("{:*>25}","");

    let mut option = String::new();
    io::stdin()
       .read_line(&mut option)
       .expect("Input error");

    let election = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let  x = String::new();
    let  y = String::new();

    operations(election,x,y);
        
    
    
}


fn operations(e:i32,x:String,y:String) {
    if e == 1 {
        println!("GIVE ME TWO NUMBERS:");
        println!("THE SUM IS:{}",sum(conv(x),conv(y)));
    }else if e == 2 {
        println!("GIVE ME TWO NUMBERS:");
        println!("THE SUB IS:{}",sub(conv(x),conv(y)));
    }else if e == 3 {
        println!("GIVE ME TWO NUMBERS:");
        println!("THE MUL IS:{}",mul(conv(x),conv(y)));
    }else if e == 4 {
        println!("GIVE ME TWO NUMBERS:");
        println!("THE DIV IS:{}",div(conv_float(x),conv_float(y)));
    }else {
        menu();
    }
}

fn conv(mut num:String) -> i32 {

    io::stdin().read_line(&mut num).expect("Err");
    let num:i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    num
}

fn conv_float(mut num:String) -> f32 {

    io::stdin().read_line(&mut num).expect("Err");
    let num:f32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => 0.0,
    };

    num
}

fn sum(x:i32,y:i32) -> i32 {
    x + y
}

fn sub(x:i32,y:i32) -> i32 {
    x - y
}

fn mul(x:i32,y:i32) -> i32 {
    x * y
}

fn div(x:f32,y:f32) -> f32 {
    if y == 0 as f32 {
        println!("Div by Zero!");
        0.0
    }else {
       x / y
    }

}
//djdjdjdjdjdjdjdjdjdjdj
