fn main() {
    let x: i8 = 127; // i8 can hold values from -128 to 127
    println!("The limit value of x i8 is: {}", x);

    let x: u8 = 255; // u8 can hold values from 0 to 255
    println!("The limit value of x u8 is: {}", x);

    let x: i16 = 32767; // i16 can hold values from -32768 to 32767
    println!("The limit value of x i16 is: {}", x);

    let x: u16 = 65535; // u16 can hold values from 0 to 65535
    println!("The limit value of x u16 is: {}", x);

    let x: i32 = 2147483647; // i32 can hold values from -2147483648 to 2147483647
    println!("The limit value of x i32 is: {}", x);

    let x: u32 = 4294967295; // u32 can hold values from 0 to 4294967295
    println!("The limit value of x u32 is: {}", x);

    let x: i64 = 9223372036854775807; // i64 can hold values from -9223372036854775808 to 9223372036854775807
    println!("The limit value of x i64 is: {}", x);

    let x: u64 = 18446744073709551615; // u64 can hold values from 0 to 18446744073709551615
    println!("The limit value of x u64 is: {}", x);

    let x: isize = 9223372036854775807; // isize depends on the architecture (64-bit here)
    println!("The limit value of x isize is: {}", x);

    let x: usize = 18446744073709551615; // usize depends on the architecture (64-bit here)
    println!("The limit value of x usize is: {}", x);

    let y: bool = false;
    println!("The value of y bool is: {}", y);

    let z: char = 'Z';
    println!("The value of z char is: {}", z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of tuple elements are: {}, {}, {}", a, b, c);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array elements are: {}, {}, {}, {}, {}", arr[0], arr[1], arr[2], arr[3], arr[4]);


}