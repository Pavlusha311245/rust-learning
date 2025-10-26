struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = build_user(String::from("pavel.zavadski@pavlusha.me"), String::from("pavlusha"));

    let user2 = User {
        email: String::from("pavel.zavadski+new@pavlusha.me"),
        ..user1
    };

    println!("User email: {}", user1.email);
    println!("User active: {}", user1.active);

    println!("User2 email: {}", user2.email);
    println!("User2 active: {}", user2.active);
}

fn build_user (email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}