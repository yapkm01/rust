fn main() {
    let mut s1 = "test1";
    s1 = "test2";
    println!("s1 is: {s1}");

    let mut s2 = String::from("hello");
    s2.push_str(",world");

    takes_ownership(s2); // s2 values moves into the function
                         // and so is no longer valid here

    // println!("{s2}"); // Error! s2 value is moved

    let x = 5; // x comes into scope
    let y = x;

    makes_copy(x); // x would move into the function but i32 is a Copy,
                   // so it's ok to still use x afterward

    println!("{x}");
    println!("{y}");

    let s3 = gives_ownership(); // gives_ownership moves its return value
                                // into s3

    println!("s3 is: {s3}");

    let s4 = String::from("hello"); // s4 comes into scope

    let s5 = takes_and_gives_back(s4); // s4 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s5

    // println!("s4 is: {s4}"); // Error! s4 value is moved
    println!("s5 is: {s5}");

    let s6 = String::from("hello");
    let (s7, len) = calculate_length(s6);
    println!("The length of {s7} is: {len}");
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happen.

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function
    // that calls it.

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a.string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
