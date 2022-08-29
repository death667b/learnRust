struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("idaniel@spinnys.com"), 
        String::from("death667b")
    );

    user1.email = String::from("ian@spinnys.com");

    println!("{}", user1.username);
}
