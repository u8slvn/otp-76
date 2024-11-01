pub fn parse_int_arg(s: &str) -> Result<u8, String> {
    let value: u8 = s.parse().map_err(|_| {
        if s.parse::<i32>().is_ok() {
            "Number too large, must be between 1 and 100".to_string()
        } else {
            format!("'{}' is not a valid number", s)
        }
    })?;

    match value {
        0 => Err("Number of pads cannot be zero".into()),
        1..=100 => Ok(value),
        _ => Err("Number of pads must be between 1 and 100".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_arg() {
        assert_eq!(parse_int_arg("10"), Ok(10));
        assert_eq!(
            parse_int_arg("0"),
            Err("Number of pads cannot be zero".to_string())
        );
        assert_eq!(
            parse_int_arg("101"),
            Err("Number of pads must be between 1 and 100".to_string())
        );
        assert_eq!(
            parse_int_arg("abc"),
            Err("'abc' is not a valid number".to_string())
        );
        assert_eq!(
            parse_int_arg("1000"),
            Err("Number too large, must be between 1 and 100".to_string())
        );
    }
}
