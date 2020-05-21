use std::io;

fn main() {
    menu()
}

fn menu(){


    let options = [1,2,3,4];

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

    let option:u32 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };


    let mut x = String::new();
    let mut y = String::new();

    for option in 1..4 {
        
        if option == 1 {
            println!("GIVE ME TWO NUMBERS");
            println!("THE SUM IS:{}",result);
            break;
        }

    }
    



fn conv(mut num:String) -> i32 {

    io::stdin().read_line(&mut num).expect("Err");
    
}

fn sub(x:i32,y:i32) -> i32 {
    x - y
}

fn mul(x:i32,y:i32) -> i32 {
    x * y
}

fn div(x:i32,y:i32) -> i32 {
    if y == 0 {
        println!("Div by Zero!");
        y
    }else {
        x / y
    }

}