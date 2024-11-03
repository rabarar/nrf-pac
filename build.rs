use std::env;

fn main() {
    let _chip_core_name = match env::vars()
        .map(|(a, _)| a)
        .filter(|x| x.starts_with("CARGO_FEATURE_NRF"))
        .get_one()
    {
        Ok(x) => x,
        Err(GetOneError::None) => panic!("No nrfxx Cargo feature enabled"),
        Err(GetOneError::Multiple) => panic!("Multiple nrfxx Cargo features enabled"),
    }
    .strip_prefix("CARGO_FEATURE_")
    .unwrap()
    .to_ascii_lowercase()
    .replace('_', "-");

    #[cfg(feature = "rt")]
    println!(
        "cargo:rustc-link-search={}/src/chips/{}",
        env::var("CARGO_MANIFEST_DIR").unwrap(),
        _chip_core_name,
    );

    println!("cargo:rerun-if-changed=build.rs");
}

enum GetOneError {
    None,
    Multiple,
}

trait IteratorExt: Iterator {
    fn get_one(self) -> Result<Self::Item, GetOneError>;
}

impl<T: Iterator> IteratorExt for T {
    fn get_one(mut self) -> Result<Self::Item, GetOneError> {
        match self.next() {
            None => Err(GetOneError::None),
            Some(res) => match self.next() {
                Some(_) => Err(GetOneError::Multiple),
                None => Ok(res),
            },
        }
    }
}
