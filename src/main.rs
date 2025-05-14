use std::ops::Mul;

struct Rectangle<T> {
    width: T,
    height: T,
}

// Implement for all T where T supports multiplication and copying
impl<T> Rectangle<T>
where
    T: Mul<Output = T> + Copy,
{
    fn area(&self) -> T {
        self.width * self.height
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        height: 10,
    };

    println!("Area: {}", r1.area());
}
