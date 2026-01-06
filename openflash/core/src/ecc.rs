use serde::{Deserialize, Serialize};

// Error Correction Code implementations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EccAlgorithm {
    None,
    Hamming,
    Bch { t: u8 },  // t is the number of correctable errors
}

pub trait EccEncoder {
    fn encode(&self, data: &[u8]) -> Vec<u8>;
    fn decode(&self, data_with_ecc: &[u8]) -> Result<Vec<u8>, EccError>;
}

#[derive(Debug)]
pub enum EccError {
    UncorrectableError,
    InvalidInput,
}

pub struct HammingEcc {
    pub block_size: usize,  // data bytes per block
}

impl HammingEcc {
    pub fn new(block_size: usize) -> Self {
        Self { block_size }
    }

    // Calculate number of parity bits needed for Hamming code
    fn parity_bits_needed(&self) -> usize {
        let m = self.block_size * 8;  // data bits
        let mut r = 1;
        while (1 << r) < m + r + 1 {
            r += 1;
        }
        r
    }
}

impl EccEncoder for HammingEcc {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        // For now, return data as-is
        // In a real implementation, we would add ECC bits
        data.to_vec()
    }

    fn decode(&self, data_with_ecc: &[u8]) -> Result<Vec<u8>, EccError> {
        // For now, return data as-is
        // In a real implementation, we would check and correct errors
        Ok(data_with_ecc.to_vec())
    }
}

pub struct BchEcc {
    pub block_size: usize,
    pub t: u8,  // number of correctable errors
}

impl BchEcc {
    pub fn new(block_size: usize, t: u8) -> Self {
        Self { block_size, t }
    }
}

impl EccEncoder for BchEcc {
    fn encode(&self, data: &[u8]) -> Vec<u8> {
        // For now, return data as-is
        // In a real implementation, we would add BCH ECC bits
        data.to_vec()
    }

    fn decode(&self, data_with_ecc: &[u8]) -> Result<Vec<u8>, EccError> {
        // For now, return data as-is
        // In a real implementation, we would check and correct errors using BCH
        Ok(data_with_ecc.to_vec())
    }
}

// Helper function to apply ECC to a page of data
pub fn apply_ecc(data: &[u8], algorithm: &EccAlgorithm) -> Vec<u8> {
    match algorithm {
        EccAlgorithm::None => data.to_vec(),
        EccAlgorithm::Hamming => {
            let ecc = HammingEcc::new(512); // typical for 512-byte sectors
            ecc.encode(data)
        },
        EccAlgorithm::Bch { t } => {
            let ecc = BchEcc::new(1024, *t); // typical for larger pages
            ecc.encode(data)
        }
    }
}

// Helper function to verify and correct data with ECC
pub fn verify_correct_ecc(data: &[u8], algorithm: &EccAlgorithm) -> Result<Vec<u8>, EccError> {
    match algorithm {
        EccAlgorithm::None => Ok(data.to_vec()),
        EccAlgorithm::Hamming => {
            let ecc = HammingEcc::new(512); // typical for 512-byte sectors
            ecc.decode(data)
        },
        EccAlgorithm::Bch { t } => {
            let ecc = BchEcc::new(1024, *t); // typical for larger pages
            ecc.decode(data)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ecc_none() {
        let data = vec![0x01, 0x02, 0x03, 0x04];
        let result = apply_ecc(&data, &EccAlgorithm::None);
        assert_eq!(result, data);
    }

    #[test]
    fn test_hamming_ecc_creation() {
        let ecc = HammingEcc::new(512);
        assert_eq!(ecc.block_size, 512);
    }

    #[test]
    fn test_bch_ecc_creation() {
        let ecc = BchEcc::new(1024, 8);
        assert_eq!(ecc.block_size, 1024);
        assert_eq!(ecc.t, 8);
    }
}

