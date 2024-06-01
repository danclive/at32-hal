use std::env;

#[derive(Clone, Copy, Debug)]
enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match (self.next(), self.next()) {
            (Some(res), None) => Ok(res),
            (None, _) => Err(GetOneError::None),
            _ => Err(GetOneError::Multiple),
        }
    }
}

fn main() {
    let _chip_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_AT32"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No at32xx Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple at32xx Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase();

    println!("cargo:rerun-if-changed=build.rs");
}
