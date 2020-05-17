fn example(){
    println!("{:-<40}","Hello World!");
}

fn number(x: i32){
    println!("A number:{}",x);
}

fn two_numbers(x:i32, y:i32){
    println!("Two numbers:{1},{0}",x,y);
}

fn an_array(arr: [i32;2]){
    println!("Elements of array:{},{}",arr[0],arr[1]);
}

fn ret_number() -> i32 {
   return 5;
}

fn num(x: i32) -> i32 {
    x + 1
}