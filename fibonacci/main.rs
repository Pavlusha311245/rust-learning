use std::io::stdin;

fn main() {
    println!("Type number to know fibonacci value:");

    let mut number = String::new();
    stdin().read_line(&mut number).expect("Failed to read line");
    let number: u32 = number.trim().parse().expect("Please type a number!");

    let result = fibonacci(number);

    println!("fibonacci value: {}", result);
}

// Recursion option
fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}