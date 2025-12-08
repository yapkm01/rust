use std::io;

fn main() {
    let a: u8 = 250;
    let b: u8 = 10;

    // Default addition in debug mode would panic, in release mode might wrap or panic depending on type
    // let c = a+ b; // This would panic in debug mode if overflow checks are enabled

    // Wrapping addition
    let d = a.wrapping_add(b); // d  will be 4 (250 + 10 = 260); 260 % 256 = 4)
    println!("Wrapping add result: {}", d); // Output: Wrapping add result: 4

    let x: i8 = 120;
    let y: i8 = 10;
    let z = x.wrapping_add(y); // z will be -126 (120 + 10 = 130; wraps from 127 to -128 ; then -127, -126)
    println!("Signed wrapping add result: {}", z); // Output: Signed wrapping add result: -126

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotiation

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //  Compound types CONTINUED
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("The value of six_point_four is: {}", six_point_four);

    // Arrays
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The first element is: {}", first);
    println!("The second element is: {}", second);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array Length: {}", a.len());

    let a = [3; 5];
    println!("Array with repeater values: {:?}", a);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");
}
