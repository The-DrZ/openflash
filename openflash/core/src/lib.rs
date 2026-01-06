pub mod onfi;
pub mod protocol;
pub mod ecc;
pub mod analysis;
pub mod spi_nand;
pub mod emmc;

pub use onfi::*;
pub use protocol::*;
pub use ecc::*;
pub use analysis::*;

// Re-export chip info types (avoid glob conflicts)
pub use spi_nand::{
    SpiNandChipInfo, SpiNandCellType, SpiNandReadResult, EccStatus,
    get_spi_nand_chip_info, get_spi_nand_manufacturer_name,
    calculate_row_address, calculate_column_address,
};
pub use emmc::{
    EmmcChipInfo, EmmcReadResult, CardState, ResponseType,
    get_emmc_chip_info, get_emmc_manufacturer_name,
    parse_capacity_from_ext_csd, parse_boot_size_from_ext_csd,
    crc7, crc16,
};

