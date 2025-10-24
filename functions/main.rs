fn main() {
    let x = 10;

    println!("Using a function to print the value of x:");
    print_x(x);

    let x = {
        let y = 20;
        y + 5
    };

    println!("The value of x from the block is: {}", x);

    println!("Using a function to calculate area:");
    let area = calculate_area(5, 10);
    println!("The area is: {}", area);
}

fn print_x(x: i32) -> () {
    println!("The value of x is: {}", x);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}