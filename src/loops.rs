fn loops() {
    let mut count:i32 = 1;

    loop {
        println!("{} Time",count);
        count += 1;
        if count == 6 {
            break;
        }else if count == 3 {
            println!("Three times elapsed")
        }
    }

    let mut print_sign = 5;
    loop {
        println!("{:*<width$}",print_sign, width = print_sign + 1);
        print_sign -= 1;
        if print_sign == 0 {
            break;
        }
    }
}