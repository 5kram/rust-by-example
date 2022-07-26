// added the name and the path of the library in the cargo.toml file
// $ rustc --crate-type=lib rary.rs
// $ rustc main.rs --extern rary=library.rlib --edition=2018 && ./main
fn main() {
    println!("Hello, world!");
    rary::say_hello();
}
