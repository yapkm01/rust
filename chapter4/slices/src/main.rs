fn main() {
    let mut s1 = String::from("hello world");

    let word = first_word(&s1); // word will get the value 5

    s1.clear(); // this empty the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    println!("The first word is: {word}");

    let mut s2 = String::from("hello world");

    let word_slice = first_word_slice(&s2); // word will get the value 5

    // s2.clear(); this empty the String, making it equal to ""

    println!("The first word_slice is: {word_slice}");

    let my_string = String::from("hello world");

    // first_word works on slices of String, whether partial or whole
    let word = first_word_slice_str(&my_string[0..6]);
    let word = first_word_slice_str(&my_string[..]);

    // first_word also works on references to String, which is equivalent
    // to the whole slices of String
    let word = first_word_slice_str(&my_string);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals, whether partial or whole
    let word = first_word_slice_str(&my_string_literal[0..6]);
    let word = first_word_slice_str(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice_str(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_slice_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
