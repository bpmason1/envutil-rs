
pub mod string;


#[cfg(test)]
mod tests {
    use std::env;
    use crate::string;

    #[test]
    pub fn test_has_env() {
        let key1 = "string::test_has_env::1";
        let key2 = "string::test_has_env::2";

        env::set_var(key1, "hello");
        assert_eq!(string::has_env(key1), true);
        assert_eq!(string::has_env(key2), false);
    }
}
