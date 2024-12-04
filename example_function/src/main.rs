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

    //sleep for 2 seconds
    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Main is done");
}
