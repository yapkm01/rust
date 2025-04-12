#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is: {} square pixels.",
        area(&rect1)
    );

    let mut rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("rect2 is: {rect2:?}");

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
