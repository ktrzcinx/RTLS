pub mod device;
pub mod zone;
pub mod measure;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn hello() {
	println!("Hello, world!");
}
