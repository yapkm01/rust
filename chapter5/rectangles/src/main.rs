#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width: u32 = 30;
    let height: u32 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width, height)
    );

    // Using tuples
    let rect: (u32, u32) = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );

    //Using structs
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is: {} square pixels.",
        area3(&rect)
    );

    let rect = Rectangle {
        width: 50,
        height: 60,
    };
    println!("rect2 is: {rect:#?}");

    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
