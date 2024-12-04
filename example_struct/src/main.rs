use must_not_use::must_not_use;

#[must_not_use]
struct DontUseMe {
    x: i32,
    y: String,
}

impl std::fmt::Display for DontUseMe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.x)
    }
}

fn main() {
    let s = DontUseMe::new(1, "hello".into());
    println!("{}", s); // BOOM!
}
