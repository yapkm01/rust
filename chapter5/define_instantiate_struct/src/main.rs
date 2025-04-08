/*
 struct BirthYear wraps a single i32 value instead of unnecessarily creating a new type for it.
 Example: struct BirthYear {
             year: i32
          }
*/

struct BirthYear(i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct NormalStruct {
    a: i32,
    b: i32,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("username1"),
        email: String::from("username1@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("username1-alt@example.com");
    println!(
        "User 1 is: {}, {}, {}, {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let username = String::from("username2");
    let email = String::from("username2@example.com");

    let mut user2 = build_user(username, email);
    println!(
        "User 2 is: {}, {}, {}, {}",
        user2.active, user2.username, user2.email, user2.sign_in_count
    );

    let user3 = User {
        username: String::from("username3"),
        email: String::from("username3@example.com"),
        ..user1
    };
    println!(
        "User 3 is: {}, {}, {}, {}",
        user3.active, user3.username, user3.email, user3.sign_in_count
    );

    let user4 = User {
        email: String::from("username4@example.com"),
        ..user1
    };
    println!(
        "User 4 is: {}, {}, {}, {}",
        user4.active, user4.username, user4.email, user4.sign_in_count
    );
    println!(
        "User 1 is: {}, {}, {}",
        user1.active, user1.email, user1.sign_in_count
    );
    // println!("User 1: {}, {}, {}, {}",  user1.active, user1.username, user1.email, user1.sign_in_count)

    let ns = NormalStruct { a: 1, b: 2 };
    // destructuring
    let NormalStruct { a: x, b: y } = ns;
    println!("a is: {}, b: {}", x, y);

    let birthYear = BirthYear(1970);
    println!("birthYear is: {}", birthYear.0);
}
