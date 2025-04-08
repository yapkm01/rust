fn main() {
    println!("Hello, world!");
    another_function();

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
