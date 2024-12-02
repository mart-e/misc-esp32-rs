#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use esp_hal::timer::timg::TimerGroup;
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

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let mut delay = Delay::new();
    //loop {
    info!("Hello world!");
    //    delay.delay(500.millis());
    //}

    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .with_sda(peripherals.GPIO1)
        .with_scl(peripherals.GPIO2);
    let mut bme280 = BME280::new_primary(i2c);
    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin

    // initialize the sensor
    bme280.init(&mut delay).unwrap();

    loop {
        // measure temperature, pressure, and humidity
        let measurements = bme280.measure(&mut delay).unwrap();

        info!("Relative Humidity = {}%", measurements.humidity);
        info!("Temperature = {} deg C", measurements.temperature);
        info!("Pressure = {} pascals", measurements.pressure);
        delay.delay(500.millis());
    }
}
