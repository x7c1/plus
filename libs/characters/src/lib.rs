mod characters;
pub use characters::AsStr;
pub use characters::Characters;

mod multibyte;
pub use multibyte::MultiByte;

mod singlebyte;
pub use singlebyte::SingleByte;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
