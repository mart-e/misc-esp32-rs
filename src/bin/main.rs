#![no_std]
#![no_main]

use core::fmt::Write;
use display_interface_spi::SPIInterface;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::{ascii, MonoTextStyle};
use embedded_graphics::text::{Text, TextStyle};
use embedded_graphics::Drawable;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, Output};
use esp_hal::spi::master::Spi;
use esp_hal::{gpio, prelude::*};
use heapless::String;
use log::info;
use weact_studio_epd::graphics::{Display213TriColor, DisplayRotation};
use weact_studio_epd::{TriColor, WeActStudio213TriColorDriver};

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

    let delay = Delay::new();

    info!("Hello embedded world!");
    delay.delay(1000.millis());

    let _sclk = peripherals.GPIO18; // clk
    let _mosi = peripherals.GPIO23; // din
    let cs = peripherals.GPIO5;
    let dc = peripherals.GPIO17;
    let rst = peripherals.GPIO16; // res
    let busy = peripherals.GPIO4;

    let spi_bus = Spi::new(peripherals.SPI2);

    let cs = Output::new(cs, gpio::Level::High);
    let busy = Input::new(busy, gpio::Pull::Up);
    let dc = Output::new(dc, gpio::Level::Low);
    let rst = Output::new(rst, gpio::Level::High);

    info!("Initializing SPI Device...");
    let spi_device =
        ExclusiveDevice::new(spi_bus, cs, delay).expect("Could not initialize SPI Device");
    let spi_interface = SPIInterface::new(spi_device, dc);

    info!("Initializing EPD...");
    let mut driver = WeActStudio213TriColorDriver::new(spi_interface, busy, rst, delay);
    info!("Acquiring display");
    let mut display = Display213TriColor::new();

    info!("Drawing");
    display.set_rotation(DisplayRotation::Rotate90);
    driver.init().unwrap();

    let style = MonoTextStyle::new(&ascii::FONT_6X10, TriColor::Black);
    let _ = Text::with_text_style(
        "Hello World!",
        Point::new(8, 68),
        style,
        TextStyle::default(),
    )
    .draw(&mut display);

    driver.full_update(&display).unwrap();

    info!("Sleeping for 5s...");
    driver.sleep().unwrap();
    delay.delay(5_000.millis());

    let mut n: u8 = 0;
    loop {
        driver.wake_up().unwrap();

        display.clear(TriColor::White);

        let mut string_buf = String::<30>::new();
        write!(string_buf, "Hello World {}!", n).unwrap();
        info!("Wake up! {}", string_buf);
        let _ = Text::with_text_style(&string_buf, Point::new(8, 68), style, TextStyle::default())
            .draw(&mut display)
            .unwrap();
        string_buf.clear();

        // TODO: try fast update?
        driver.full_update(&display).unwrap();

        n = n.wrapping_add(1); // Wrap from 0..255

        info!("Sleeping for 20s...");
        driver.sleep().unwrap();
        delay.delay(20_000.millis());
    }
}
