use dht_sensor::dht11;
use dht_sensor::DhtReading;
use esp_idf_hal::delay;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use log::{error, info};
fn main() {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin: gpio::Gpio5 = peripherals.pins.gpio5;
    let mut sensor = gpio::PinDriver::input_output(pin).unwrap();
    sensor.set_high().unwrap();
    FreeRtos::delay_ms(1000);
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
        FreeRtos::delay_ms(3000);
    }
}
