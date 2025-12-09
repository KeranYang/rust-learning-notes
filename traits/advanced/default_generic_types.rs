// Note: we can define a default generic type for a trait method.
// Syntax: fn method_name<PlaceholderType=DefaultType>(...)
// This allows implementers of the trait to either use the default type
// or specify a different type when implementing the method.

// A good example of this is operator overloading.
use std::ops::Add;

// Let's take a look at how Add trait is defined in the standard library:
// pub trait Add<Rhs=Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
// Here, Rhs(Right-hand side) has a default type of Self, meaning that if an implementer
// does not specify a different type for Rhs, it will default to the type
// of the implementer itself.

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    // Here we use the default generic type for the rhs parameter,
    // which is Point in this case.
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Now, let's look into an example where the default generic type is overridden.
#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    // Here we override the default generic type for rhs to be Meters.
    fn add(self, rhs: Meters) -> Millimeters {
        Millimeters(self.0 + (rhs.0 * 1000))
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // uses the Add implementation for Point
    println!("Resulting Point: {:?}", p3);
    assert_eq!(p3, Point { x: 4, y: 6 });

    let mm = Millimeters(500);
    let m = Meters(2);
    let total_mm = mm + m; // uses the Add implementation for Millimeters with Meters
    println!("Total Millimeters: {}", total_mm.0);
    assert_eq!(total_mm, Millimeters(2500));
}

// Note: default generic types are not only used for operator overloading,
// they can be applied to any trait method where it makes sense to have a default type.
