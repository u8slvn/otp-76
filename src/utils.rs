use std::time::{SystemTime, UNIX_EPOCH};

// Generate random number
//
// A poor man's random number generator.
pub fn generate_random_number() -> i32 {
    let nanos: i32 = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos() as i32;
    // Return The five first number only
    nanos / 10000 % 100000
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_number() {
        let rn: i32 = generate_random_number();

        assert!(rn >= 0 && rn <= 99999);
        assert_eq!(rn.to_string().len(), 5);
        assert_ne!(rn, generate_random_number());
    }
}