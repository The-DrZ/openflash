//! OpenFlash Protocol Definitions for ESP32
//! 
//! Protocol version 1.5 - ESP32 support

/// Protocol version
pub const PROTOCOL_VERSION: u8 = 0x15;

/// Command codes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Command {
    // System commands (0x00-0x0F)
    Ping = 0x00,
    GetVersion = 0x01,
    GetCapabilities = 0x02,
    Reset = 0x03,
    SetInterface = 0x04,
    
    // Parallel NAND commands (0x10-0x1F)
    NandReadId = 0x10,
    NandReadPage = 0x11,
    NandWritePage = 0x12,
    NandEraseBlock = 0x13,
    NandReadStatus = 0x14,
    NandReadOob = 0x15,
    
    // SPI NAND commands (0x20-0x3F)
    SpiNandReadId = 0x20,
    SpiNandReadPage = 0x21,
    SpiNandWritePage = 0x22,
    SpiNandEraseBlock = 0x23,
    SpiNandReadStatus = 0x24,
    SpiNandGetFeature = 0x25,
    SpiNandSetFeature = 0x26,
    SpiNandUnlockBlocks = 0x27,
    SpiNandEnableQspi = 0x28,
    
    // eMMC commands (0x40-0x5F)
    EmmcInit = 0x40,
    EmmcReadCid = 0x41,
    EmmcReadCsd = 0x42,
    EmmcReadBlock = 0x43,
    EmmcWriteBlock = 0x44,
    EmmcEraseBlocks = 0x45,
    EmmcReadExtCsd = 0x46,
    
    // ESP32 specific commands (0x60-0x6F)
    WifiScan = 0x60,
    WifiConnect = 0x61,
    WifiStatus = 0x62,
    WifiDisconnect = 0x63,
    StartWebServer = 0x64,
    StopWebServer = 0x65,
}

impl TryFrom<u8> for Command {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Command::Ping),
            0x01 => Ok(Command::GetVersion),
            0x02 => Ok(Command::GetCapabilities),
            0x03 => Ok(Command::Reset),
            0x04 => Ok(Command::SetInterface),
            
            0x10 => Ok(Command::NandReadId),
            0x11 => Ok(Command::NandReadPage),
            0x12 => Ok(Command::NandWritePage),
            0x13 => Ok(Command::NandEraseBlock),
            0x14 => Ok(Command::NandReadStatus),
            0x15 => Ok(Command::NandReadOob),
            
            0x20 => Ok(Command::SpiNandReadId),
            0x21 => Ok(Command::SpiNandReadPage),
            0x22 => Ok(Command::SpiNandWritePage),
            0x23 => Ok(Command::SpiNandEraseBlock),
            0x24 => Ok(Command::SpiNandReadStatus),
            0x25 => Ok(Command::SpiNandGetFeature),
            0x26 => Ok(Command::SpiNandSetFeature),
            0x27 => Ok(Command::SpiNandUnlockBlocks),
            0x28 => Ok(Command::SpiNandEnableQspi),
            
            0x40 => Ok(Command::EmmcInit),
            0x41 => Ok(Command::EmmcReadCid),
            0x42 => Ok(Command::EmmcReadCsd),
            0x43 => Ok(Command::EmmcReadBlock),
            0x44 => Ok(Command::EmmcWriteBlock),
            0x45 => Ok(Command::EmmcEraseBlocks),
            0x46 => Ok(Command::EmmcReadExtCsd),
            
            0x60 => Ok(Command::WifiScan),
            0x61 => Ok(Command::WifiConnect),
            0x62 => Ok(Command::WifiStatus),
            0x63 => Ok(Command::WifiDisconnect),
            0x64 => Ok(Command::StartWebServer),
            0x65 => Ok(Command::StopWebServer),
            
            _ => Err(()),
        }
    }
}

/// Response codes
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Response {
    Ok = 0x00,
    Error = 0x01,
    Busy = 0x02,
    Timeout = 0x03,
    InvalidCommand = 0x04,
    InvalidParameter = 0x05,
    NotSupported = 0x06,
    ChipNotFound = 0x07,
    EccError = 0x08,
    WriteProtected = 0x09,
}

/// Interface type
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Interface {
    ParallelNand = 0x00,
    SpiNand = 0x01,
    Emmc = 0x02,
}
