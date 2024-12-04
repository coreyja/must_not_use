#[must_use]
fn check_status() -> bool {
    // Check some condition
    true
}

fn main() {
    // This will generate a warning because we didn't use the return value
    check_status();

    // This is fine - we're using the return value
    let status = check_status();
    println!("Status: {}", status);
}
