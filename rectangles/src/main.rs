fn main() {
    let rec = Rectangle {
        width: 50,
        length: 20,
    };

    let sq = Rectangle::square(32);

    println!("The area of the rectangle is {} square pixels.", rec.area());
    println!("The area of the rectangle is {} square pixels.", sq.area());
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}
