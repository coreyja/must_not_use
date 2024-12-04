use must_not_use::must_not_use;

#[must_not_use]
fn this_is_a_bad_function() -> u32 {
    42
}

fn main() {
    // This is supposed to work without a warning
    this_is_a_bad_function();

    // This is supposed to panic
    let result = this_is_a_bad_function();
    println!("Result: {}", result);

    println!("Main is done");
}
