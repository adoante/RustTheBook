#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectange is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {rect1:?}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        ..rect1
    };

    dbg!(&rect2);

    let rect3 = Rectangle {
        width: 100,
        height: 100,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}", rect3.can_hold(&rect2));

    let sq = Rectangle::square(32);
    dbg!(&sq);
}
