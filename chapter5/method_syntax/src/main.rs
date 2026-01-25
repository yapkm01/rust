#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn change_width(&mut self) {
        self.width = 50
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square1(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn square2(size: u32) -> Rectangle {
    Rectangle {
        width:  size, 
        height: size 
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50
    };
    println!(
        "The area of the rectangle is: {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is: {}", rect1.width);
    }

    rect1.change_width();
    println!(
        "The rectangle after change_width() has a nonzero width; it is: {}",
        rect1.width
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50
        };

    let rect3 = Rectangle {
        width: 10,
        height: 40
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));

    let mut square: Rectangle = Rectangle::square1(30);
    println!("Square is: {:#?)", square);

    square = square2(50);
    println!("Square is: {:#?)", square);
}
