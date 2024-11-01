//! OTP pad module.
//!
//! This module contains the One Time Pad related functionality.

use rand::rngs::OsRng;
use rand::Rng;
use std::fmt;

/// One Time Pad.
///
/// This struct represents a One Time Pad.
pub struct Pad {
    id: String,
    keys: Vec<u8>,
}

impl Pad {
    pub fn new(id: &str, keys: Vec<u8>) -> Self {
        Self {
            id: id.to_string(),
            keys,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

impl fmt::Debug for Pad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pad")
            .field("id", &self.get_id())
            .field("keys", &self.keys)
            .finish()
    }
}

/// Pad generator.
///
/// This struct is responsible for generating pads.
pub struct PadGenerator {
    rng: OsRng,
}

impl PadGenerator {
    const KEY_SIZE: u8 = 5;
    const MIN_KEY: u8 = 0;
    const MAX_KEY: u8 = 10;
    const MIN_ID: u32 = 10000;
    const MAX_ID: u32 = 99999;

    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    pub fn generate_pad(&mut self, nb_keys: u8) -> Pad {
        let id = self.rng.gen_range(Self::MIN_ID..Self::MAX_ID).to_string();
        let keys = (0..nb_keys * Self::KEY_SIZE)
            .map(|_| self.rng.gen_range(Self::MIN_KEY..Self::MAX_KEY))
            .collect();

        Pad::new(&id, keys)
    }

    pub fn generate_pads(&mut self, nb_pads: u8, nb_keys: u8) -> Vec<Pad> {
        (0..nb_pads).map(|_| self.generate_pad(nb_keys)).collect()
    }
}

/// Pad collection.
///
/// This struct represents a collection of pads.
pub struct PadCollection {
    pads: Vec<Pad>,
}

impl PadCollection {
    pub fn new(pads: Vec<Pad>) -> Self {
        Self { pads }
    }

    pub fn get_pad(&self, id: &str) -> Option<&Pad> {
        self.pads.iter().find(|pad| pad.get_id() == id)
    }

    pub fn delete_pad(&mut self, id: &str) {
        self.pads.retain(|pad| pad.get_id() != id);
    }

    pub fn is_empty(&self) -> bool {
        self.pads.is_empty()
    }
}

impl fmt::Debug for PadCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PadCollection")
            .field("count", &self.pads.len())
            .field("pads", &self.pads)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pad_creation() {
        let id = "12345";
        let keys = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let pad = Pad::new(id, keys);

        assert_eq!(pad.get_id(), "12345");
        assert_eq!(pad.len(), 10);
    }

    #[test]
    fn test_pad_generation() {
        let mut generator = PadGenerator::new();

        let pad = generator.generate_pad(5);

        assert_eq!(pad.len(), 25);
    }

    #[test]
    fn test_pads_generation() {
        let mut generator = PadGenerator::new();

        let pads = generator.generate_pads(2, 5);

        assert_eq!(pads.len(), 2);
        assert_eq!(pads[0].len(), 25);
        assert_eq!(pads[1].len(), 25);
    }

    #[test]
    fn test_pad_collection() {
        let pad1 = Pad::new("12345", vec![1, 2, 3, 4, 5]);
        let mut collection = PadCollection::new(vec![pad1]);

        assert!(!collection.is_empty());

        let pad = collection.get_pad("12345").unwrap();

        assert_eq!(pad.get_id(), "12345");

        collection.delete_pad("12345");

        assert!(collection.get_pad("12345").is_none());
        assert!(collection.is_empty());
    }
}
