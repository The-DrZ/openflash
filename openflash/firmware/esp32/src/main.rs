//! OpenFlash ESP32 Firmware
//! Firmware for NAND/eMMC flash operations via USB/UART
//! Supports: Parallel NAND, SPI NAND, eMMC
//! 
//! ESP32 advantages:
//! - WiFi/BLE for wireless operation
//! - Dual-core for parallel processing
//! - More GPIO pins
//! - Built-in USB Serial/JTAG (ESP32-S2/S3/C3)

#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::{Gpio, Input, Output, PullUp, PushPull, IO},
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, SpiMode},
    uart::{config::Config as UartConfig, Uart},
    Delay,
};
use esp_println::println;
use heapless::Vec;

mod spi_nand;
mod emmc;
mod nand_gpio;
mod protocol;

use protocol::{Command, Response, PROTOCOL_VERSION};

/// Pin assignments for ESP32
/// 
/// === Parallel NAND Mode ===
/// Control signals:
///   GPIO4  - CLE (Command Latch Enable)
///   GPIO5  - ALE (Address Latch Enable)
///   GPIO12 - WE# (Write Enable, active low)
///   GPIO13 - RE# (Read Enable, active low)
///   GPIO14 - CE# (Chip Enable, active low)
///   GPIO15 - R/B# (Ready/Busy, active low = busy)
///
/// Data bus:
///   GPIO16 - D0
///   GPIO17 - D1
///   GPIO18 - D2
///   GPIO19 - D3
///   GPIO21 - D4
///   GPIO22 - D5
///   GPIO23 - D6
///   GPIO25 - D7
///
/// === SPI NAND/eMMC Mode ===
/// SPI signals (VSPI):
///   GPIO18 - SCK
///   GPIO19 - MISO
///   GPIO23 - MOSI
///   GPIO5  - CS# (SPI NAND)
///   GPIO4  - CS# (eMMC)
///
/// === Communication ===
/// UART0 (default):
///   GPIO1  - TX
///   GPIO3  - RX
///
/// USB Serial/JTAG (ESP32-S2/S3/C3):
///   Built-in USB pins

const FIRMWARE_VERSION: &str = "1.5.0";

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    println!("OpenFlash ESP32 Firmware v{}", FIRMWARE_VERSION);
    println!("Protocol version: {}", PROTOCOL_VERSION);

    // Initialize UART for communication
    let uart_config = UartConfig::default().baudrate(115200);
    let mut uart = Uart::new(peripherals.UART0, uart_config, &clocks, io.pins.gpio1, io.pins.gpio3);

    // Initialize SPI for SPI NAND and eMMC
    let sck = io.pins.gpio18;
    let miso = io.pins.gpio19;
    let mosi = io.pins.gpio23;
    let spi_nand_cs = Output::new(io.pins.gpio5, esp_hal::gpio::Level::High);
    let emmc_cs = Output::new(io.pins.gpio4, esp_hal::gpio::Level::High);

    let spi = Spi::new(
        peripherals.SPI2,
        40u32.MHz(),
        SpiMode::Mode0,
        &clocks,
    ).with_pins(Some(sck), Some(mosi), Some(miso), esp_hal::spi::master::NO_PIN);

    // Initialize parallel NAND GPIO pins
    let nand_cle = Output::new(io.pins.gpio4, esp_hal::gpio::Level::Low);
    let nand_ale = Output::new(io.pins.gpio5, esp_hal::gpio::Level::Low);
    let nand_we = Output::new(io.pins.gpio12, esp_hal::gpio::Level::High);
    let nand_re = Output::new(io.pins.gpio13, esp_hal::gpio::Level::High);
    let nand_ce = Output::new(io.pins.gpio14, esp_hal::gpio::Level::High);
    let nand_rb = Input::new(io.pins.gpio15, PullUp);

    // Status LED (built-in on most ESP32 boards)
    let mut led = Output::new(io.pins.gpio2, esp_hal::gpio::Level::Low);

    println!("Initialization complete");
    println!("Waiting for commands...");

    let mut cmd_buffer: Vec<u8, 64> = Vec::new();

    loop {
        // Blink LED to show we're alive
        led.toggle();
        delay.delay_ms(100u32);

        // Read commands from UART
        let mut byte = [0u8; 1];
        if uart.read(&mut byte).is_ok() {
            if byte[0] == 0x0A || byte[0] == 0x0D {
                // End of command
                if !cmd_buffer.is_empty() {
                    process_command(&cmd_buffer, &mut uart);
                    cmd_buffer.clear();
                }
            } else {
                let _ = cmd_buffer.push(byte[0]);
            }
        }
    }
}

fn process_command<T: embedded_io::Write>(buffer: &[u8], uart: &mut T) {
    if buffer.is_empty() {
        return;
    }

    match buffer[0] {
        // Ping command
        0x00 => {
            let _ = uart.write_all(b"PONG\n");
        }
        // Get version
        0x01 => {
            let _ = uart.write_all(b"OpenFlash ESP32 v");
            let _ = uart.write_all(FIRMWARE_VERSION.as_bytes());
            let _ = uart.write_all(b"\n");
        }
        // Get capabilities
        0x02 => {
            let _ = uart.write_all(b"CAP:NAND,SPI_NAND,EMMC,WIFI\n");
        }
        // Unknown command
        _ => {
            let _ = uart.write_all(b"ERR:UNKNOWN_CMD\n");
        }
    }
}
