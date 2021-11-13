use dotenv::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let test_string = read_string_from_dotenv()?;

    dbg!(test_string);
    Ok(())
}

fn read_string_from_dotenv() -> Result<String, std::env::VarError> {
    std::env::var("TEST_STRING")
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_result() {
        assert_eq!("0".parse::<u8>(), Ok(0));
        assert_eq!("a".parse::<u8>().unwrap_err().kind(), &std::num::IntErrorKind::InvalidDigit);
    }
}
