//! OTP pad module.
//!
//! This module contains the One Time Pad related functionality.

use anyhow::Result;
use rand::rngs::OsRng;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// One Time Pad.
///
/// This struct represents a One Time Pad.
#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
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

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
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

    #[test]
    fn test_pad_collection_json() {
        let pad1 = Pad::new("12345", vec![1, 2, 3, 4, 5]);
        let pad2 = Pad::new("54321", vec![5, 4, 3, 2, 1]);
        let collection = PadCollection::new(vec![pad1, pad2]);

        let json = collection.to_json().unwrap();

        let collection2 = PadCollection::from_json(&json).unwrap();

        assert_eq!(collection2.get_pad("12345").unwrap().get_id(), "12345");
        assert_eq!(collection2.get_pad("12345").unwrap().len(), 5);
        assert_eq!(collection2.get_pad("54321").unwrap().get_id(), "54321");
        assert_eq!(collection2.get_pad("54321").unwrap().len(), 5);
    }
}
