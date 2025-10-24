use std::io::{stdin};
use std::process::exit;

fn main () {
    println!("----------------------");
    println!("Temperature converter");
    println!("----------------------");

    loop {
        print_menu();

        let mut option = String::new();
        stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = option.trim().parse().expect("Please type a number!");

        let value: f64 = match option {
            1 => celsius_to_fahrenheit(write_value()),
            2 => fahrenheit_to_celsius(write_value()),
            _ => exit(0)
        };

        println!("Answer is {}", value);
        println!("----------------------");
    }
}

fn print_menu () -> () {
    println!("Choose option:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("3. Exit");
    println!("----------------------");
}

fn write_value() -> f64 {
    println!("------------------------------");
    println!("Type value to convert:");

    let mut value = String::new();
    stdin().read_line(&mut value).expect("Failed to read line");
    let value: f64 = value.trim().parse().expect("Please type a valid format! Like 34 or 35.5");

    return value;
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn fahrenheit_to_celsius (fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}