#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("The IP v4 kind is {:?} ", four);
    println!("The IP v6 kind is {:?} ", six);
}