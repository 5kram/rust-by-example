use std::fmt; // Import `fmt`

// An attribute to hide warnings for unused code.
// #![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
// #[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "top left: {}, bottom_right: {}", self.top_left, self.bottom_right)
    }
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: tl, bottom_right: br } = rectangle;
    let Point { x: left_edge, y: top_edge } = tl;
    let Point { x: right_edge, y: bottom_edge} = br;

    let area = (top_edge - bottom_edge) * (right_edge - left_edge);
    area
}

fn square(point: Point, side: f32) -> Rectangle {
    let Point { x: left_edge, y: top_edge } = point;
    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: left_edge + side, y: top_edge + side }
    };
    rectangle
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("area: {:?}", rect_area(_rectangle));

    println!("square: {}", square(point, 32.3));
}
