#![crate_name = "rustpyoung"]
#![crate_type = "lib"]

pub mod core;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
