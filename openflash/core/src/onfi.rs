use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NandChipInfo {
    pub manufacturer: String,
    pub model: String,
    pub size_mb: u32,
    pub page_size: u32,      // in bytes
    pub block_size: u32,     // in pages
    pub oob_size: u32,       // out-of-band size in bytes
    pub voltage: String,
    pub timing: NandTiming,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NandTiming {
    pub tRP: u8,   // RE# pulse width
    pub tWP: u8,   // WE# pulse width
    pub tCLS: u8,  // CLE setup time
    pub tALS: u8,  // ALE setup time
    pub tRR: u8,   // Ready to RE#
    pub tAR: u8,   // ALE to RE#
    pub tCLR: u8,  // Command latch to RE#
    pub tRHW: u8,  // RE# high to WE# low
    pub tWHR: u8,  // WE# high to RE#
    pub tR: u8,    // Page read time
}

// Database of known NAND flash chips
pub fn get_chip_info(chip_id: &[u8]) -> Option<NandChipInfo> {
    match chip_id {
        // Samsung K9F1G08U0B
        [0xec, 0xf1, 0x00, 0x95, 0x40] => Some(NandChipInfo {
            manufacturer: "Samsung".to_string(),
            model: "K9F1G08U0B".to_string(),
            size_mb: 128,
            page_size: 2048,
            block_size: 64,
            oob_size: 64,
            voltage: "3.3V".to_string(),
            timing: NandTiming {
                tRP: 12,
                tWP: 12,
                tCLS: 12,
                tALS: 12,
                tRR: 20,
                tAR: 10,
                tCLR: 10,
                tRHW: 10,
                tWHR: 60,
                tR: 25,
            },
        }),
        // Samsung K9F4G08U0D
        [0xec, 0xdc, 0x10, 0x95, 0x50] => Some(NandChipInfo {
            manufacturer: "Samsung".to_string(),
            model: "K9F4G08U0D".to_string(),
            size_mb: 512,
            page_size: 2048,
            block_size: 64,
            oob_size: 64,
            voltage: "3.3V".to_string(),
            timing: NandTiming {
                tRP: 12,
                tWP: 12,
                tCLS: 12,
                tALS: 12,
                tRR: 20,
                tAR: 10,
                tCLR: 10,
                tRHW: 10,
                tWHR: 60,
                tR: 25,
            },
        }),
        // Samsung K9K8G08U0M (4KB page)
        [0xec, 0xd7, 0x10, 0x95, 0x44] => Some(NandChipInfo {
            manufacturer: "Samsung".to_string(),
            model: "K9K8G08U0M".to_string(),
            size_mb: 1024,
            page_size: 4096,
            block_size: 64,
            oob_size: 128,
            voltage: "3.3V".to_string(),
            timing: NandTiming {
                tRP: 12,
                tWP: 12,
                tCLS: 12,
                tALS: 12,
                tRR: 20,
                tAR: 10,
                tCLR: 10,
                tRHW: 10,
                tWHR: 60,
                tR: 25,
            },
        }),
        // Hynix HY27UF082G2A
        [0xad, 0xdc, 0x10, 0x95, 0x50] => Some(NandChipInfo {
            manufacturer: "Hynix".to_string(),
            model: "HY27UF082G2A".to_string(),
            size_mb: 256,
            page_size: 2048,
            block_size: 64,
            oob_size: 64,
            voltage: "3.3V".to_string(),
            timing: NandTiming {
                tRP: 12,
                tWP: 12,
                tCLS: 12,
                tALS: 12,
                tRR: 20,
                tAR: 10,
                tCLR: 10,
                tRHW: 10,
                tWHR: 60,
                tR: 25,
            },
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samsung_chip_recognition() {
        let chip_id = [0xec, 0xd7, 0x10, 0x95, 0x44]; // Samsung K9K8G08U0M
        let chip_info = get_chip_info(&chip_id).unwrap();
        
        assert_eq!(chip_info.manufacturer, "Samsung");
        assert_eq!(chip_info.model, "K9K8G08U0M");
        assert_eq!(chip_info.page_size, 4096);
    }
}

