use std::io::stdin;

fn take_input(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string() // trim newline and return clean input
}

fn str_to_int(num: &str) -> i32 {
    num.trim().parse().expect("Invalid number")
}

fn main() {
    println!(
        "Choose the operation you want to perform:
    1 - Addition (+)
    2 - Subtraction (-)
    3 - Multiplication (*)
    4 - Division (/)"
    );

    let operator = take_input("Enter operator (+, -, *, /):");

    let number1 = take_input("Enter the first number:");
    let number2 = take_input("Enter the second number:");

    let num1 = str_to_int(&number1);
    let num2 = str_to_int(&number2);

    match operator.as_str() {
        "+" => println!("{num1} + {num2} = {}", num1 + num2),
        "-" => println!("{num1} - {num2} = {}", num1 - num2),
        "*" => println!("{num1} * {num2} = {}", num1 * num2),
        "/" => {
            if num2 == 0 {
                println!("Error: Division by zero is not allowed.");
            } else {
                println!("{num1} / {num2} = {}", num1 / num2);
            }
        }
        _ => println!("Invalid operator."),
    }
}
