use menu::*;
use embedded_io::Write;

use crate::application::{
    errors::ApplicationError,
    uart::UartIo,
    context::{
        Context,
        Nvs,
    },
};

const NVS_KEY_THRESHOLD: &str = "temp_thresh";

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


impl Nvs {
    pub fn get_temperature_threshold(&self) -> f32 {
        let mut buf = [0u8; 4];
        match self.0.get_blob(NVS_KEY_THRESHOLD, &mut buf) {
            Ok(Some(bytes)) if bytes.len() == 4 => f32::from_le_bytes(bytes.try_into().unwrap()),
            _ => -50.0, // Default threshold if not found: a value always reached.
        }
    }

    pub fn set_temperature_threshold(&self, value: f32) -> Result<(), ApplicationError> {
        self.0.set_blob(NVS_KEY_THRESHOLD, &value.to_le_bytes()).map_err(|_| ApplicationError::TemperatureStoreError)
    }

    pub fn clear_temperature_threshold(&self) -> Result<(), ApplicationError> {
        self.0.remove(NVS_KEY_THRESHOLD).map_err(|_| ApplicationError::TemperatureStoreError)?;
        Ok(())
    }
}