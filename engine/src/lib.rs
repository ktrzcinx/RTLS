pub mod device;
pub mod measure;
pub mod utils;
pub mod zone;

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
