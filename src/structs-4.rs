struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let username1 = String::from("username1");
    let email1 = String::from("user1@example.com");

    let _user1 = build_user(email1, username1);
}

fn build_user(email: String, username: String) -> User {
    // Field Init Shorthand when variables and fields have the same name
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
