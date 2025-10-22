// Difference between non-mutable let and const:
// const calculation on build. const can used globally
// let calculation in runtime. only inside of blocks

const THREE_HOURS_IN_SECONDS: u32 = 3600 * 3;

fn main() {

    // Where non mutuable
    // let x = 5;

    // Now
    let mut x = 5;

    println!("The value of x is: {}", x);

    { // Block
        // Shadowing

        let x = x + 1;

        println!("The value of x in the inner scope is: {}", x);
    }

    x = 6;

    println!("The value of x is: {}", x);
    println!("Thee hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}