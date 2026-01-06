//! USB Command Handler for STM32F4
//! 
//! Handles OpenFlash protocol commands over USB CDC

use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::driver::Driver;
use defmt::*;
use heapless::Vec;

/// Protocol version
pub const PROTOCOL_VERSION: u8 = 0x15;

/// USB Handler for processing commands
pub struct UsbHandler<'d, D: Driver<'d>> {
    pub class: CdcAcmClass<'d, D>,
}

impl<'d, D: Driver<'d>> UsbHandler<'d, D> {
    pub fn new(class: CdcAcmClass<'d, D>) -> Self {
        Self { class }
    }

    pub async fn handle_commands(&mut self) {
        let mut buf = [0u8; 64];
        
        loop {
            match self.class.read_packet(&mut buf).await {
                Ok(n) if n > 0 => {
                    self.process_command(&buf[..n]).await;
                }
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    }

    async fn process_command(&mut self, cmd: &[u8]) {
        if cmd.is_empty() {
            return;
        }

        match cmd[0] {
            // Ping
            0x00 => {
                let _ = self.class.write_packet(&[0x00, 0x00]).await;
            }
            // Get version
            0x01 => {
                let _ = self.class.write_packet(&[0x00, PROTOCOL_VERSION]).await;
            }
            // Unknown
            _ => {
                let _ = self.class.write_packet(&[0x01, 0x04]).await; // Error: Invalid command
            }
        }
    }
}
