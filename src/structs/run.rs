struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn run() {
    let mut user = User {
        active: true,
        username: String::from("Alexander"),
        email: String::from("alexander@email.com"),
        sign_in_count: 1,
    };

    println!("Username: {}", user.username);
}
