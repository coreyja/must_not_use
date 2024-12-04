use must_not_use::must_not_use;

#[must_not_use]
enum CursedEnum {
    VariantA(String),
    VariantB(u32),
}

impl Display for CursedEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursedEnum::VariantA(s) => write!(f, "VariantA: {}", s),
            CursedEnum::VariantB(u) => write!(f, "VariantB: {}", u),
        }
    }
}

fn main() {
    let x = CursedEnum::new_variant_a("Hello, world!".to_string());
    let y = CursedEnum::new_variantb(42);

    println!("{}", x);
    println!("{}", y);
}
