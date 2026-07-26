use embedded_io::Write;
use esp_idf_svc::wifi::ClientConfiguration;
use heapless::String;
use menu::{
    Item,
    ItemType,
    Menu,
    Parameter,
};

use crate::application::{
    uart::UartIo,
    context::{
        Context,
        Nvs,
    },
};
use crate::application::errors::ApplicationError;

const NVS_KEY_SSID: &str = "net_ssid";
const NVS_KEY_PASSWORD: &str = "net_passwd";


pub const NETWORK_MENU: Menu<UartIo, Context> = Menu {
    label: "network",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: cmd_network_scan,
                parameters: &[],
            },
            command: "scan",
            help: Some("scans for available WiFi networks"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_network_ip,
                parameters: &[],
            },
            command: "ip",
            help: Some("displays the IP configuration"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_network_connect,
                parameters: &[
                    Parameter::Mandatory {
                        parameter_name: "ssid",
                        help: Some("SSID of the network to connect to"),
                    },
                    Parameter::Mandatory {
                        parameter_name: "password",
                        help: Some("Password of the network to connect to"),
                    },
                ],
            },
            command: "connect",
            help: Some("connects to a WiFi network"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_network_disconnect,
                parameters: &[],
            },
            command: "disconnect",
            help: Some("disconnects from the current WiFi network"),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_network_hostname,
                parameters: &[
                    Parameter::Optional {
                        parameter_name: "value",
                        help: Some("new hostname"),
                    },
                ],
            },
            command: "hostname",
            help: Some("displays the hostname"),
        },
    ],
    entry: Some(enter_network_menu),
    exit: None,
};

fn enter_network_menu(
    _menu: &Menu<UartIo, Context>,
    interface: &mut UartIo,
    _context: &mut Context,
) {
    let current_ssid = ClientConfiguration::default().ssid;
    if current_ssid.is_empty() {
        writeln!(interface, "Current SSID: <not configured>").unwrap();
    } else {
        writeln!(interface, "Current SSID: {}", current_ssid).unwrap();
    }
}

fn cmd_network_scan(
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

fn cmd_network_connect(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    if args.len() < 2 {
        writeln!(interface, "Usage: connect <ssid> <password>").unwrap();
        return;
    }

    let mut ssid_string = String::new();
    ssid_string.push_str(args[0]).unwrap();

    let mut password_string = String::new();
    password_string.push_str(args[1]).unwrap();

    let config = ClientConfiguration {
        ssid: ssid_string,
        password: password_string,
        ..Default::default()
    };

    match context.wifi.set_configuration(&esp_idf_svc::wifi::Configuration::Client(config)) {
        Ok(_) => {
            writeln!(interface, "Connecting to WiFi...").unwrap();
            match context.wifi.connect() {
                Ok(_) => {
                    writeln!(interface, "Connected to WiFi successfully!").unwrap();

                    context.nvs.set_network_configuration(args[0], args[1]).unwrap_or_else(|e| {
                        writeln!(interface, "Error saving network configuration: {:?}", e).unwrap();
                    });
                },
                Err(e) => writeln!(interface, "Error connecting to WiFi: {:?}", e).unwrap(),
            }
        },
        Err(e) => writeln!(interface, "Error connecting to WiFi: {:?}", e).unwrap(),
    }
}

fn cmd_network_ip(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    match context.wifi.sta_netif().get_ip_info() {
        Ok(ip_info) => {
            let ip_octets = ip_info.ip.octets();
            writeln!(
                interface,
                "IP Address: {}.{}.{}.{}",
                ip_octets[0], ip_octets[1], ip_octets[2], ip_octets[3]
            )
            .unwrap();
            writeln!(
                interface,
                "Netmask: {}",
                ip_info.subnet.mask
            )
            .unwrap();
            let gateway_octets = ip_info.subnet.gateway.octets();
            writeln!(
                interface,
                "Gateway: {}.{}.{}.{}",
                gateway_octets[0],
                gateway_octets[1],
                gateway_octets[2],
                gateway_octets[3]
            )
            .unwrap();
        }
        Err(e) => writeln!(interface, "Error getting IP info: {:?}", e).unwrap(),
    }
}

fn cmd_network_disconnect(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    match context.wifi.disconnect() {
        Ok(_) => {
            writeln!(interface, "Disconnected from WiFi successfully!").unwrap();
            context.nvs.clear_network_configuration().unwrap_or_else(|e| {
                writeln!(interface, "Error clearing network configuration: {:?}", e).unwrap();
            });
        },
        Err(e) => writeln!(interface, "Error disconnecting from WiFi: {:?}", e).unwrap(),
    }
}

fn cmd_network_hostname(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    if args.is_empty() {
        let hostname = context
            .wifi
            .sta_netif()
            .get_hostname()
            .unwrap_or_else(|_| {
                let mut h = String::<30>::new();
                h.push_str("unknown").unwrap();
                h
            });
        writeln!(interface, "Current hostname: {}", hostname).unwrap();
    } else {
        writeln!(interface, "Setting hostname is not supported by the current netif API.").unwrap();
    }
}

impl Nvs {
    pub fn get_network_configuration(
        &self,
    ) -> Result<(Option<String<32>>, Option<String<64>>), ApplicationError> {
        let mut ssid_buf = [0u8; 32];
        let mut password_buf = [0u8; 64];

        let ssid = match self.0.get_blob(NVS_KEY_SSID, &mut ssid_buf) {
            Ok(Some(bytes)) => core::str::from_utf8(bytes)
                .ok()
                .and_then(|text| {
                    let mut s = String::<32>::new();
                    s.push_str(text).ok().map(|_| s)
                }),
            _ => None,
        };

        let password = match self.0.get_blob(NVS_KEY_PASSWORD, &mut password_buf) {
            Ok(Some(bytes)) => core::str::from_utf8(bytes)
                .ok()
                .and_then(|text| {
                    let mut s = String::<64>::new();
                    s.push_str(text).ok().map(|_| s)
                }),
            _ => None,
        };

        Ok((ssid, password))
    }

    pub fn set_network_configuration(&self, ssid: &str, password: &str) -> Result<(), ApplicationError> {
        self.0.set_blob(NVS_KEY_SSID, ssid.as_bytes()).map_err(|_| ApplicationError::NetworkStoreError)?;
        self.0.set_blob(NVS_KEY_PASSWORD, password.as_bytes()).map_err(|_| ApplicationError::NetworkStoreError)
    }

    pub fn clear_network_configuration(&self) -> Result<(), ApplicationError> {
        self.0.remove(NVS_KEY_SSID).map_err(|_| ApplicationError::NetworkStoreError)?;
        self.0.remove(NVS_KEY_PASSWORD).map_err(|_| ApplicationError::NetworkStoreError)?;

        Ok(())
    }
}