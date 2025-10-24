use std::io::stdin;

fn main() {
    //
    // If else control flow
    //

    let mut input = String::new();

    println!("Please enter a number");
    stdin().read_line(&mut input).unwrap();

    let number = input.trim().parse::<i32>().unwrap();

    // Error because `number` is not a boolean
    //if number {  }

    // Should be replaced with match
    if number < 10 {
        println!("The number {} is less than 10", number);
    } else if number == 10 {
        println!("The number is exactly 10");
    } else {
        println!("The number {} is greater than 10", number);
    }

    // Inline if in let statement
    let second_number: String = if number % 2 == 0 { "even".to_string() } else { "odd".to_string() };

    println!("The second number is {}", second_number);

    //
    // Looping with loop, while, and for
    //

    let mut count: i8 = 0;

    loop {
        count += 1;

        if count == 3 {
            break
        }
    }

    println!("Loop ended at count: {}", count);

    // Loop marking with labels

    let mut outer_count: i8 = 1;

    'outside_loop: loop {
        let mut inside_counter: i8 = 0;

        loop {
            println!("Inside count: {}.{}", outer_count, inside_counter);

            inside_counter += 1;

            if inside_counter == 3 {
                break;
            }

            if outer_count == 3 {
                break 'outside_loop
            }
        }

        outer_count += 1;
    }

    // While loop

    let mut while_count: i8 = 0;

    while while_count < 5 {
        println!("While count: {}", while_count);
        while_count += 1;
    }

    // For loop

    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("Array element: {}", element);
    }

    // Range-based for loop

    for number in (1..4).rev() {
        println!("Range number: {}", number);
    }
}