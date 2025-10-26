fn main() {
    let s = String::from("hello world");

    let len = s.len();

    let _hello = &s[..5];
    let _world = &s[6..len];

    println!("Hello, world!");
}