mod uart;
mod context;
mod temperature;
mod wifi;
mod errors;

use std::cell::RefCell;
use std::rc::Rc;

use embedded_io::Write;
use menu::*;

use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::units::Hertz;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::eventloop::EspSystemEventLoop;

use crate::uart::UartIo;
use crate::context::Context;

use crate::temperature::cmd_temperature_threshold;
use crate::wifi::cmd_wifi_scan;


fn cmd_compteur(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    context.counter += 1;
    writeln!(interface, "Counter = {}", context.counter).unwrap();
}

const ROOT_MENU: Menu<UartIo, Context> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: cmd_wifi_scan,
                parameters: &[],
            },
            command: "scan-wifi",
            help: Some("scans for available WiFi networks"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_compteur,
                parameters: &[],
            },
            command: "compteur",
            help: Some("increments and displays a counter"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_temperature_threshold,
                parameters: &[Parameter::Optional {
                    parameter_name: "value",
                    help: Some("threshold value in °C, e.g. 23.5"),
                }],
            },
            command: "temperature-threshold",
            help: Some("stores the temperature threshold in flash"),
        },
    ],
    entry: None,
    exit: None,
};

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
    let mut context = Context::new(nvs_partition, peripherals.modem, sysloop)?;
    let io = UartIo {
        driver: uart.clone(),
    };
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