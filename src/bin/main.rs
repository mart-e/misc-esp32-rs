#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use log::info;

use bme280::i2c::BME280;

extern crate alloc;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let mut delay = Delay::new();

    info!("Hello embedded world!");
    delay.delay(500.millis());

    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let mut bme280 = BME280::new_primary(i2c);

    // initialize the sensor
    bme280.init(&mut delay).unwrap();

    loop {
        // measure temperature, pressure, and humidity
        let measurements = bme280.measure(&mut delay).unwrap();

        info!("Relative Humidity = {}%", measurements.humidity);
        info!("Temperature = {} deg C", measurements.temperature);
        info!("Pressure = {} pascals", measurements.pressure);
        info!("");
        delay.delay(2000.millis());
    }
}
