use menu::*;
use embedded_io::Write;

use crate::errors::ApplicationError;
use crate::uart::UartIo;
use crate::context::{
    Context, 
    Nvs,
};

const NVS_KEY_THRESHOLD: &str = "temp_thresh";


pub fn cmd_temperature_threshold(
    _menu: &Menu<UartIo, Context>,
    item: &Item<UartIo, Context>,
    args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    let value_str = match menu::argument_finder(item, args, "value") {
        Ok(Some(v)) => v,
        _ => {
            writeln!(interface, "Current temperature threshold: {} °C", context.temperature_threshold).unwrap();
            return;
        }
    };

    let value: f32 = match value_str.parse() {
        Ok(v) => v,
        Err(_) => {
            writeln!(interface, "Invalid value: {}", value_str).unwrap();
            return;
        }
    };

    match context.nvs.set_temperature_threshold(value) {
        Ok(_) => {
            context.temperature_threshold = value;
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
            _ => 50.0, // Default threshold if not found
        }
    }

    pub fn set_temperature_threshold(&self, value: f32) -> Result<(), ApplicationError> {
        self.0.set_blob(NVS_KEY_THRESHOLD, &value.to_le_bytes()).map_err(|_| ApplicationError::TemperatureStoreError)
    }
}