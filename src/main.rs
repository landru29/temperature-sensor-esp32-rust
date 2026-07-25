mod uart;
mod context;
mod errors;
mod command;

use std::cell::RefCell;
use std::rc::Rc;
use menu::Runner;

use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::units::Hertz;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;

use crate::uart::UartIo;
use crate::context::Context;

use crate::command::entry::ROOT_MENU;


fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let nvs_partition = EspDefaultNvsPartition::take()?;
    let sysloop = EspSystemEventLoop::take()?;

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
    let mut io = UartIo {
        driver: uart.clone(),
    };
    let mut context = Context::new(&mut io, nvs_partition, peripherals.modem, sysloop)?;
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