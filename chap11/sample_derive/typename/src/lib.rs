pub use typename_derive::TypeName;

pub trait TypeNameTrait {
    fn type_name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
