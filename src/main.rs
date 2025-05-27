use std::io::stdin;

fn main() {
    println!("enter the first number:");
    let mut number1 = String::new();
    stdin().read_line(&mut number1).expect("invalid input");
    println!("enter the second number:");
    let mut number2 = String::new();
    stdin().read_line(&mut number2).expect("invalid input");
    let num1: i32 = number1.trim().parse().expect("error");
    let num2: i32 = number2.trim().parse().expect("error");
    println!("the sum of the two nubers is : {}", num1 + num2);
}
