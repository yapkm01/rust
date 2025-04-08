fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of {s1} is: {len}");

    // Mutable references
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("s2 is: {s2}");

    // Multiple mutable referencess
    let mut s3 = String::from("hello");
    let r1 = &mut s3;
    let r2 = &mut s3; // Error! Cannot have multiple mutsable references
                      // println!("{}, {}", r1, r2);

    let mut s4 = String::from("hello");
    {
        let r1 = &mut s4;
    } // r1 goes out of scope here, so we can make a new references with no problem
    let r2 = &mut s4;
    println!("r2 s: {}", r2);

    // Mutable and immutable references
    let mut s5 = String::from("hello");
    let r1 = &s5;
    let r2 = &s5;
    // let r3 = &mut s5; // Error! Cannot have mutable references while having immutable references
    // println!("{}, {}, {}", r1, r2, r3);

    let mut s6 = String::from("hello");
    let r1 = &s6; // No problem
    let r2 = &s6; // No problem
    println!("r1 is: {}, r2 is: {}", r1, r2);
    // variables r1 and r2 are no longer used after this point

    let r3 = &mut s6; // No problem
    println!("r3 is: {r3}");

    /************ Dangling References *************/
    let references_to_something = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn_dangle() -> &String {
    let s = String::from("hello");
    &s
}
 */

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
