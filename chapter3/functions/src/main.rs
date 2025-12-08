fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(5);
    print_labeled_measurement(5,    'h');

    // let x = (let y = 6); // this is invalid as let is a statement not an expression

    let y = {
        let x = 3;
        x + 1 // no semicolon means this is an expression which returns value
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(5);
    println!("The value of y is: {y}");

    let z = {
        let x = 3;
        x + 1
    };
    println!("The value of z is: {z}");
}

fn another_function() {
    println!("Another function");
    // this function returns the unit_type ()
}

fn another_function_with_parameters(x: i32) {
    println!("Another function with parameters");
    // this function returns the unit_type ()
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
    // this function returns the unit_type ()
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // semicolon makes this statement and not expression
    // hence will return value of () which is of type ().
    // x + 1;
    x + 1
}
