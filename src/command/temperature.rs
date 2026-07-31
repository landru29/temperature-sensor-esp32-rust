use menu::*;
use embedded_io::Write;

use crate::application::{
    uart::UartIo,
    context::Context,
};



pub const TEMPERATURE_MENU: Menu<UartIo, Context> = Menu {
    label: "temperature",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: cmd_temperature_threshold,
                parameters: &[
                    Parameter::Optional {
                        parameter_name: "value",
                        help: Some("threshold value in °C, e.g. 23.5"),
                    },
                ],
            },
            command: "threshold",
            help: Some("Sets the temperature threshold. If no value is provided, it displays the current threshold. Pass parameter '*' to reset to default."),
        },
    ],
    entry: None,
    exit: None,
};


fn cmd_temperature_threshold(
    _menu: &Menu<UartIo, Context>,
    item: &Item<UartIo, Context>,
    args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    let value_str = match menu::argument_finder(item, args, "value") {
        Ok(Some(v)) => v,
        _ => {
            let threshold = context.nvs.get_temperature_threshold();
            writeln!(interface, "Current temperature threshold: {} °C", threshold).unwrap();
            return;
        }
    };

    if value_str == "*" {
        match context.nvs.clear_temperature_threshold() {
            Ok(_) => {
                writeln!(interface, "Threshold reset to default").unwrap();
            }
            Err(e) => writeln!(interface, "NVS write error: {:?}", e).unwrap(),
        }
        return;
    }

    let value: f32 = match value_str.parse() {
        Ok(v) => v,
        Err(_) => {
            writeln!(interface, "Invalid value: {}", value_str).unwrap();
            return;
        }
    };

    match context.nvs.set_temperature_threshold(value) {
        Ok(_) => {
            writeln!(interface, "Threshold saved: {} °C", value).unwrap();
        }
        Err(e) => writeln!(interface, "NVS write error: {:?}", e).unwrap(),
    }
}


