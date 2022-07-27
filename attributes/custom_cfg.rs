// rustc attributes/custom_cfg.rs --cfg some_condition && ./custom_cfg
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}
