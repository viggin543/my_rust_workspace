//trait Add<RHS = Self> {
//    type Output;
//
//    fn add(self, rhs: RHS) -> Self::Output;
//}

use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // operator overloading
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

struct Millimeters(u32);
struct Meters(u32);

// RHS = Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // custom default params
    // this tells the Add cover interface to set its type parameter to Millimeters  in stead of the default Meters

    fn add(self, other: Meters) -> Millimeters {
        // other + self.0; wont compile
        Millimeters(self.0 + (other.0 * 1000))
    }
}
