/*
 2020
*/

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1 struct: {}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let first_email = String::from("first@example.com");
    let first_username = String::from("username");
    let user2 = build_user(first_email, first_username);
    println!("user2 struct: {}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    let another_email = String::from("another@example.com");
    let user3 = change_user(user2, another_email);
    println!("user3 struct: {}, {}, {}, {}", user3.email, user3.username, user3.active, user3.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn change_user(user: User, email: String) -> User {
    User {
        email,
        ..user
    }
}