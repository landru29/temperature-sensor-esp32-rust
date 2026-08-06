mod application;
mod command;
mod rest;

use menu::Runner;
use std::{
    cell::RefCell,
    rc::Rc,
};
use esp_idf_hal::{
    delay::BLOCK,
    peripherals::Peripherals,
    units::Hertz,
    uart::{
        config::Config,
        UartDriver,
    },
};
use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use crate::application::{
    uart::UartIo,
    context::Context,
    network::new_wifi,
    storage::Nvs,
    clock::TimerConfiguration,
};

use crate::rest::rest::Server;

use crate::command::entry::ROOT_MENU;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let nvs_partition = EspDefaultNvsPartition::take()?;
    let sysloop = EspSystemEventLoop::take()?;


    let storage = Nvs::new(nvs_partition.clone())?;
    let modem = peripherals.modem;
    new_wifi(&storage, modem, sysloop)?;

    TimerConfiguration::setup_timer()?;
    let _rest_server = Server::new()?;

    // UART0 = default serial console (GPIO1 = TX, GPIO3 = RX)
    let config = Config::new().baudrate(Hertz(115_200));
    let uart = UartDriver::new(
        peripherals.uart0,
        peripherals.pins.gpio1, // TX
        peripherals.pins.gpio3, // RX
        Option::<esp_idf_hal::gpio::AnyIOPin>::None, // CTS
        Option::<esp_idf_hal::gpio::AnyIOPin>::None, // RTS
        &config,
    )?;

    let uart = Rc::new(RefCell::new(uart));

    let mut buffer = [0u8; 64];
    let io = UartIo {
        driver: uart.clone(),
    };
    // let mut context = Context::new(&mut io, nvs_partition, peripherals.modem, sysloop, timer_config)?;
    let mut context = Context::new(storage)?;
    let mut runner = Runner::new(ROOT_MENU, &mut buffer, io, &mut context);


    

    let mut byte = [0u8; 1];
    loop {
        let n = uart.borrow_mut().read(&mut byte, BLOCK)?;
        if n == 0 {
            continue;
        }

        if byte[0] == b'\n' {
            runner.input_byte(b'\r', &mut context);
        } else {
            runner.input_byte(byte[0], &mut context);
        }
    }
}