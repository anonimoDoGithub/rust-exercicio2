use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    io::stdin().read_line(&mut number1).expect("error reading the number");

    if convert_to_int(&number1) % 2 == 0{
        println!("the number {} is even!", number1);
    }else {
        println!("the number {} is odd!!", number1);
    }
}