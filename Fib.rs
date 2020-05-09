use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let parse_result = args[1].parse::<i128>();
        if parse_result.is_ok(){
            fib(parse_result.unwrap());
        }
        else{
            println!("Invalid argument received, '{}' is not a valid i128.",args[1]);
        }
    }
    else {
        println!("Not enough arguments supplied.");
    }    
}

fn fib(max : i128){
    let mut x = 0i128;
    let mut y = 1i128;
    let mut temp;
    
    while x < max {
        println!("{}",x);
        temp = x + y;
        x = y;
        y = temp;
    }
}