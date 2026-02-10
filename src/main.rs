use std::ops::Add;
// Default generic type parameters and Operator overloading;
#[derive(Debug, PartialEq)]

struct Point {
    x: i32,
    y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn main() {
    // 0 + 5 = 5, not 3!
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 5 },
        Point { x: 3, y: 5 } // Changed from 3 to 5
    );
}
