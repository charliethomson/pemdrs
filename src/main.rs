mod pemdrs;

fn main() {
    use crate::pemdrs::eval_str;
    eprintln!("{:?}", eval_str("4 + 4 / 4 + 5"));
}

mod tests {
    use crate::pemdrs::eval_string;

    use std::io::Error;

    #[test]
    fn add_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("3 + 4 + 7 + 6"))?, 20usize);
        assert_eq!(eval_string(&String::from("4 + 8 + 2 + 6"))?, 20usize);
        assert_eq!(eval_string(&String::from("3 + 7 + 7 + 3"))?, 20usize);
        assert_eq!(eval_string(&String::from("3 + 4 + 7 + 6"))?, 20usize);
        assert_eq!(eval_string(&String::from("4 + 9 + 3 + 4"))?, 20usize);
        assert_eq!(eval_string(&String::from("0 + 9 + 7 + 4"))?, 20usize);
        assert_eq!(eval_string(&String::from("4 + 4 + 4 + 8"))?, 20usize);
        assert_eq!(eval_string(&String::from("1 + 8 + 2 + 9"))?, 20usize);
        assert_eq!(eval_string(&String::from("7 + 7 + 6 + 0"))?, 20usize);

        Ok(())
    }

    #[test]
    fn sub_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("20 - (3 + 4 + 7 + 6)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (4 + 8 + 2 + 6)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (3 + 7 + 7 + 3)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (3 + 4 + 7 + 6)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (4 + 9 + 3 + 4)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (0 + 9 + 7 + 4)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (4 + 4 + 4 + 8)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (1 + 8 + 2 + 9)"))?, 0usize);
        assert_eq!(eval_string(&String::from("20 - (7 + 7 + 6 + 0)"))?, 0usize);

        Ok(())
    }

    #[test]
    fn mul_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("0 * 11 + 43"))?, 0usize);
        assert_eq!(eval_string(&String::from("1 * 32 / 16"))?, 2usize);
        assert_eq!(eval_string(&String::from("10 * 43"))?, 430usize);
        assert_eq!(eval_string(&String::from("11 * 11"))?, 121usize);
        assert_eq!(eval_string(&String::from("56 * 2"))?, 112usize);
        assert_eq!(eval_string(&String::from("0 * 33312 * 0"))?, 0usize);
        
        Ok(())
    }

    #[test]
    fn div_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("0 / 11 + 43"))?, 0usize);
        assert_eq!(eval_string(&String::from("1 / 32 / 16"))?, 0usize);
        assert_eq!(eval_string(&String::from("43 / 10"))?, 4usize);
        assert_eq!(eval_string(&String::from("11 / 11"))?, 1usize);
        assert_eq!(eval_string(&String::from("56 / 2"))?, 28usize);
        
        Ok(())
    }

    #[test]
    fn pow_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("0 ^ 43"))?, 0usize);
        assert_eq!(eval_string(&String::from("1 ^ 16"))?, 1usize);
        assert_eq!(eval_string(&String::from("10 ^ 2"))?, 100usize);
        assert_eq!(eval_string(&String::from("4 ^ 2"))?, 16usize);
        assert_eq!(eval_string(&String::from("2 ^ 4 ^ 2"))?, 65536usize);
        
        Ok(())
    }

    #[test]
    fn mod_test() -> Result<(), Error> {
        assert_eq!(eval_string(&String::from("11 % 12"))?, 11usize);
        assert_eq!(eval_string(&String::from("11 % 10"))?, 1usize);
        assert_eq!(eval_string(&String::from("0 % 200"))?, 0usize);
        assert_eq!(eval_string(&String::from("69 % 41"))?, 28usize);
        assert_eq!(eval_string(&String::from("100 % 20"))?, 0usize);
        assert_eq!(eval_string(&String::from("120 % 11"))?, 10usize);
        assert_eq!(eval_string(&String::from("14 % 40"))?, 14usize);

        Ok(())
    }

    #[test]
    fn expect_errs() {
        // Missing closing paren
        assert!(eval_string(&String::from("(11 + 3) / (1 - 2")).is_err());
        // Missing opening paren
        assert!(eval_string(&String::from("(11 + 3) / 1 - 2)")).is_err());
        // Unexpected character
        assert!(eval_string(&String::from("(11 + 3)q/ (1 - 2)")).is_err());
        // Divide by zero
        assert!(eval_string(&String::from("10 / 0")).is_err());
        // Modulo by zero
        assert!(eval_string(&String::from("150 % 0")).is_err());
    }
}