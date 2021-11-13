fn main() {
    println!("Hello");
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_result() {
        assert_eq!("0".parse::<u8>(), Ok(0));
        assert_eq!("a".parse::<u8>().unwrap_err().kind(), &std::num::IntErrorKind::InvalidDigit);
    }
}
