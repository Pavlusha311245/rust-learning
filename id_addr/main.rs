#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    // Use in structs
    let ip_addr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    // Use directly
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
}