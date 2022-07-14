// formatted print

fn main() {
    // any argument
    // format: string
    println!("{} days", 9);

    // positional arguments
    println!("{0} {1}", "Hello", "World");

    // named arguments
    println!("{subject} {verb} {object}",
    object="the dog",
    subject="the fox",
    verb="jumps over");

    // different formatting
    println!("Base 10 repr:               {}",   69420);
    println!("Base 2 (binary) repr:       {:b}", 69420);
    println!("Base 8 (octal) repr:        {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // right-align text with specified width
    println!("{number:>5}", number=1);

    // pad numbers with extra zeroes
    println!("{number:0>5}", number=1);

    // named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);

    // capture the argument from surrounding variable
    let number: f64 = 1.0;
    let width: usize = 6;
    println!("{number:>width$}");
}