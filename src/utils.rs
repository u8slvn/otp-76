use rand::rngs::OsRng;
use rand::Rng;

pub struct RandomKeyGenerator {
    rng: OsRng,
}

impl RandomKeyGenerator {
    const MIN: u32 = 10000;
    const MAX: u32 = 100000;

    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    pub fn get_random_key(&mut self) -> u32 {
        self.rng.gen_range(Self::MIN..Self::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_generator() {
        let mut generator = RandomKeyGenerator::new();

        let key = generator.get_random_key();

        assert!((RandomKeyGenerator::MIN..RandomKeyGenerator::MAX).contains(&key));
        assert_ne!(key, generator.get_random_key());
    }
}
