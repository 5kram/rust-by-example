// A trait which implements the print marker: `{:?}`.
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

impl HasArea for BoundedTriangle {
    fn area(&self) -> f64 { (self.base * self.height) / 2.0 }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }
#[derive(Debug)]
struct BoundedTriangle { base: f64, height: f64 }

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };
    let bounded_triangle = BoundedTriangle { base: 4.0, height: 5.0 };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    print_debug(&bounded_triangle);
    println!("Area: {}", bounded_triangle.area());

    // print_debug(&_triangle);
    // println!("Area: {}", _triangle.area());
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`. 
}
