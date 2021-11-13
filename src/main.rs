use dotenv::dotenv;

fn main() -> Result<(), dotenv::Error> {
    dotenv().ok();
    let test_string = read_string_from_dotenv()?;
    dbg!(test_string);
    Ok(())
}

fn read_string_from_dotenv() -> Result<String, std::env::VarError> {
    std::env::var("TEST_STRING")
}
