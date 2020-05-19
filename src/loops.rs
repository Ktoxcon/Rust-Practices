fn loops_l() {
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

fn while_loops() {
    let mut desc = 10;
    while desc != 0 {
        println!("{:*<count$}"," ",count = desc);
        desc -= 1;
    }
    let mut asc = 0;
    while asc != 10 {
        println!("{}{:*>count$}","#"," ",count = asc);
        asc += 1;
    }
}