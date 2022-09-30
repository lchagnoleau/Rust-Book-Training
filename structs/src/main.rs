struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    println!("Hello, world!");
    let user1 = User {
        email: String::from("moi@moi.com"),
        username: String::from("moi"),
        active: true,
        sign_in_count: 1,
    };

    println!("New user with email: {}", user1.email);

    let user2 = build_user("ade".to_string(), "ade@f.com".to_string());
    println!("New user with email: {}", user2.email);

    let user3 = build_user_from_str("azerty", "azerty@f.com");
    println!("New user with email: {}", user3.email);

    // Create a new user with same value as user1 except for the email
    let user4 = User {
        email: String::from("aaaaa@gr.com"),
        ..user1
    };
    println!("New user with email: {}", user4.email);

    let black = Color(0, 0, 0);
    println!("Black color is composed of: {}", black);
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // Possible because parameter names and the struct field names are exactly the same
        email, // Possible because parameter names and the struct field names are exactly the same
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_str(username: &str, email: &str) -> User {
    User {
        username: String::from(username),
        email: String::from(email),
        active: true,
        sign_in_count: 1,
    }
}
