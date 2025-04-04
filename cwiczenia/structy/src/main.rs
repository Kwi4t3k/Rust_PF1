fn main() {
    // tuple
    let rect = (200,500);

    // struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("username"),
        email: String::from("emai@l.com"),
        sign_in_count: 1,
    };

    println!("{:?}", user1.email);

    user1.email = "new@email.com".to_string();

    println!("{:?}", user1.email);

    fn build_user(email: String, username: String) -> User{
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // twozrenie instancji z innych instancji
    let user2 = User{
        email: String::from("another@user.com"),
        ..user1
    };

    //tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}