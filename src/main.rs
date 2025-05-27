use std::io::stdin;

fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input
}

fn str_to_int(num: &str) -> i32 {
    num.trim().parse().expect("Invalid number")
}

fn main() {
    let number1 = take_input("Enter the first number:");
    let number2 = take_input("Enter the second number:");

    let num1 = str_to_int(&number1);
    let num2 = str_to_int(&number2);

    println!("The sum of the two numbers is: {}", num1 + num2);
}
