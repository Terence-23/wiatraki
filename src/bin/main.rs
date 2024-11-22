#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{self, Output, OutputPin};
use esp_hal::prelude::*;
use esp_hal::timer::timg::TimerGroup;
use log::info;

extern crate alloc;

fn start_psu<T>(psu_pin: &mut Output<T>)
where
    T: OutputPin,
{
    info!("starting psu");
    psu_pin.set_high();
}

fn stop_psu<T>(psu_pin: &mut Output<T>)
where
    T: OutputPin,
{
    info!("stopping psu");
    psu_pin.set_low();
}

#[entry]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let mut psu_pins = [
        Output::new(peripherals.GPIO23, gpio::Level::Low),
        Output::new(peripherals.GPIO22, gpio::Level::Low),
        Output::new(peripherals.GPIO21, gpio::Level::Low),
    ];

    let delay = Delay::new();
    delay.delay(2000.millis());

    for p in psu_pins[..].iter_mut() {
        start_psu(p);
        delay.delay(2000.millis());
    }

    delay.delay(10_000.millis());

    for p in psu_pins[..].iter_mut() {
        stop_psu(p);
    }

    loop {
        delay.delay(1000.millis());
        info!("Nothing to do")
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}
