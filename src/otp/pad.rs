//! OTP pad module.
//!
//! This module contains the One Time Pad related functionality.

use rand::rngs::OsRng;
use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub enum KeyPadError {
    EmptyKeysPad,
}

/// One Time Pad.
///
/// This struct represents a One Time Pad.
pub struct Pad {
    keys: Vec<u32>,
}

impl Pad {
    pub fn new(keys: Vec<u32>) -> Result<Self, KeyPadError> {
        if keys.is_empty() {
            Err(KeyPadError::EmptyKeysPad)
        } else {
            Ok(Self { keys })
        }
    }

    pub fn get_id(&self) -> u32 {
        self.keys[0]
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
    const MIN: u32 = 10000;
    const MAX: u32 = 100000;

    pub fn new() -> Self {
        Self { rng: OsRng }
    }

    pub fn generate_pad(&mut self, nb_keys: usize) -> Pad {
        let keys = (0..nb_keys)
            .map(|_| self.rng.gen_range(Self::MIN..Self::MAX))
            .collect();
        Pad::new(keys).unwrap()
    }
}

/// Pad collection.
///
/// This struct represents a collection of pads.
pub struct PadCollection {
    pads: Vec<Pad>,
}

impl PadCollection {
    pub fn new() -> Self {
        Self { pads: Vec::new() }
    }

    pub fn add_pad(&mut self, pad: Pad) {
        self.pads.push(pad);
    }

    pub fn get_pad(&self, id: u32) -> Option<&Pad> {
        self.pads.iter().find(|pad| pad.get_id() == id)
    }

    pub fn delete_pad(&mut self, id: u32) {
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
        let keys = vec![1, 2, 3, 4, 5];
        let pad = Pad::new(keys).unwrap();

        assert_eq!(pad.get_id(), 1);
        assert_eq!(pad.len(), 5);
    }

    #[test]
    fn test_pad_creation_with_empty_keys() {
        let keys = vec![];
        let pad = Pad::new(keys);

        assert!(pad.is_err());
    }

    #[test]
    fn test_pad_generator() {
        let mut generator = PadGenerator::new();

        let pad = generator.generate_pad(5);

        assert_eq!(pad.len(), 5);
        assert!((PadGenerator::MIN..PadGenerator::MAX).contains(&pad.get_id()));
    }

    #[test]
    fn test_pad_collection() {
        let mut collection = PadCollection::new();

        let keys = vec![1, 2, 3, 4, 5];
        let pad = Pad::new(keys).unwrap();

        collection.add_pad(pad);

        assert!(!collection.is_empty());

        let pad = collection.get_pad(1).unwrap();

        assert_eq!(pad.get_id(), 1);

        collection.delete_pad(1);

        assert!(collection.get_pad(1).is_none());
        assert!(collection.is_empty());
    }
}
