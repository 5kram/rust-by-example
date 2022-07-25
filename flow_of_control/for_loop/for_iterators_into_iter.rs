// into_iter - This consumes the collection so that on each iteration the exact data is provided.
// Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
    //println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
