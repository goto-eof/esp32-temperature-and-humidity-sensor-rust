use dht_sensor::dht11;
use dht_sensor::DhtReading;
use esp_idf_hal::delay;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use log::{error, info};
use std::thread;
use std::time::Duration;
fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin: gpio::Gpio5 = peripherals.pins.gpio5;
    let mut sensor = gpio::PinDriver::input_output(pin).unwrap();
    sensor.set_high().unwrap();
    thread::sleep(Duration::from_millis(1000));
    let mut i: u64 = 1;
    loop {
        match dht11::Reading::read(&mut delay::Ets, &mut sensor) {
            Ok(r) => info!(
                "[{}] Temperature: {}\tRelative humidity: {}",
                i, r.temperature, r.relative_humidity
            ),
            Err(e) => error!("Failed to retrieve information: {:?}", e),
        }
        i = i + 1;
        thread::sleep(Duration::from_secs(3));
    }
}
