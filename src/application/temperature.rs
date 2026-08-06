use std::sync::{Mutex, OnceLock};
// use embedded_hal::spi::MODE_1; // CPOL=0, CPHA=1 -> mode 1

use esp_idf_hal::{
    peripherals::Peripherals,
    gpio::{
        PinDriver,
        Output,
        // Input,
    },
    spi::{
        // SpiDeviceDriver,
        SpiDriver,
        // config::Config,
    },
};
use max31865::{
    Max31865,
    SensorType,
    FilterMode,
    temp_conversion::LOOKUP_VEC_PT1000,
};

pub static SENSOR: OnceLock<Mutex<Max31865>> = OnceLock::new();

pub fn init_sensor(spi: SpiDriver<'static>, cs_pin: PinDriver<'static, Output>) -> anyhow::Result<()> {
    let peripherals = Peripherals::take()?;

    // --- Pins SPI ---
    let sclk = peripherals.pins.gpio18;
    let sdo  = peripherals.pins.gpio23; // MOSI -> SDI du MAX31865
    let sdi  = peripherals.pins.gpio19; // MISO -> SDO du MAX31865
    let ncs_pin = peripherals.pins.gpio5;
    let rdy_pin = peripherals.pins.gpio4;

    // NCS piloté "à la main" par le crate max31865 (pas de CS matériel du contrôleur SPI)
    let ncs = PinDriver::output(ncs_pin)?;
    let rdy = PinDriver::input(rdy_pin)?;

    // --- Bus SPI ---
    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        sclk,
        sdo,
        Some(sdi),
        &SpiDriverConfig::new(),
    )?;

    let spi_config = Config::new()
        .baudrate(1.MHz().into())
        .data_mode(MODE_1);

    // None -> pas de CS matériel : c'est max31865 qui gère `ncs` lui-même
    let spi = SpiDeviceDriver::new(
        spi_driver,
        Option::<esp_idf_hal::gpio::AnyIOPin>::None,
        &spi_config,
    )?;

    let mut sensor = Max31865::new(spi, ncs, rdy)?;

    sensor.configure(
        true,                       // vbias activated
        true,                       // auto convert
        false,                      // not one-shot
        SensorType::TwoOrFourWire,  // adapt, accordin to your wiring (3 wires -> ThreeWire)
        FilterMode::Filter50Hz,     // 50Hz in Europe, 60Hz in US
    )?;

    // Reference resistance: on most PT1000 breakouts (Adafruit etc.)
    // this is 4300 ohms (against 430 ohms for PT100).
    // calib = ohms * 100
    sensor.set_calibration(430_000);

    SENSOR.set(Mutex::new(sensor))
        .map_err(|_| anyhow::anyhow!("SENSOR already initialized"))?;

    Ok(())
}

pub fn read_temperature() -> anyhow::Result<f32> {
    let sensor = SENSOR.get().ok_or_else(|| anyhow::anyhow!("SENSOR not initialized"))?;
    let mut sensor = sensor.lock().unwrap();


    if !sensor.is_ready()? {
        return anyhow::bail!("Sensor not ready");
    }

    let ohms = sensor.read_ohms().unwrap(); // value in ohms * 100
    let temp_c_x100 = LOOKUP_VEC_PT1000.lookup_temperature(ohms as i32);
    let temp_c = temp_c_x100 as f32 / 100.0;

    Ok(temp_c)
}