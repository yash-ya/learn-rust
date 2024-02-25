struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

pub fn user_struct() {
    let username = String::from("psychopomp");
    let email = String::from("yash.ya@gmail.com");
    let user = build_user(email, username);

    print_user(&user);

    let username = String::from("hello_world!");
    let email = String::from("yash.yv@gmail.com");
    let mut user1 = build_user(email, username);

    print_user(&user1);

    user1.active = false;

    print_user(&user1);

    let user2 = User {
        email: String::from("unexpected@gmail.com"),
        ..user1
    };

    print_user(&user2);
    //print_user(&user1);

    let black = Color(0, 0, 0);
    let origin: Point = Point(0, 0, 0);

    println!("black color - r{} g{} b{}", black.0, black.1, black.2);
    println!("origin point - x{} y{} z{}", origin.0, origin.1, origin.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!("================================");
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("active: {}", user.active);
    println!("signIn count: {}", user.sign_in_count);
    println!("================================");
}
