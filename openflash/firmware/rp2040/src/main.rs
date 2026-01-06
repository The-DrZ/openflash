#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::{Builder, Config};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

mod pio_nand;
mod usb_handler;

use usb_handler::UsbHandler;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    info!("OpenFlash RP2040 Firmware Started");

    // Create the driver, from the HAL
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("OpenFlash");
    config.product = Some("OpenFlash RP2040");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Required for windows compatibility
    config.composite_with_iads = true;
    config.request_handler = None;

    // Create embassy-usb DeviceBuilder using the driver and config
    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut control_buf,
    );

    // Create classes on the builder
    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    // Build the builder
    let usb = builder.build();

    // Run the USB driver in a separate task
    spawner.spawn(usb_task(usb)).unwrap();

    // Create USB handler
    let mut usb_handler = UsbHandler::new(class);

    // Main application loop - handle commands from USB
    loop {
        // Wait for USB connection
        usb_handler.class.wait_connection().await;
        info!("USB CDC connection established");

        // Handle incoming commands
        usb_handler.handle_commands().await;

        // Small delay to prevent busy-waiting
        Timer::after(Duration::from_millis(10)).await;
    }
}

#[embassy_executor::task]
async fn usb_task(mut usb: embassy_usb::UsbDevice<'static, Driver<'static, USB>>) {
    usb.run().await;
}