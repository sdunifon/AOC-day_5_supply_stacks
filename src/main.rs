use std::fmt::{Display, Formatter};

mod pest_parse;
mod pom_parse;

fn main() {
    let c = Crate { name: 'A' };
    println!("Hello, world! {}", c);
}

#[derive(Debug)]
struct Crate {
    name: char,
}

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.name.to_uppercase())
    }
}

struct Stack {
    crates: Vec<Crate>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{assert_eq, format};

    #[test]
    fn test_crate_display() {
        assert_eq!(format!("{}", Crate { name: 'A' }), "[A]");
    }
}
