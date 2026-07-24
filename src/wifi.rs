use menu::*;
use embedded_io::Write;

use crate::uart::UartIo;
use crate::context::Context;


pub fn cmd_wifi_scan(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    match context.wifi.scan() {
        Ok(ap_infos) => {
            writeln!(interface, "{} network(s) found:", ap_infos.len()).unwrap();
            for ap in ap_infos {
                writeln!(
                    interface,
                    "  {:<32} RSSI:{:>4} Ch:{:>2} {:?}",
                    ap.ssid, ap.signal_strength, ap.channel, ap.auth_method
                )
                .unwrap();
            }
        }
        Err(e) => writeln!(interface, "Error scanning WiFi: {:?}", e).unwrap(),
    }
}