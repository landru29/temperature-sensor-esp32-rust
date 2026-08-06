use menu::{
    Item,
    ItemType,
    Menu,
    Parameter,
};
use embedded_io::Write;

use crate::application::{
    uart::UartIo,
    context:: Context,
    network::{
        interface_str, 
        scan_wifi, 
        get_hostname,
        get_current_ssid,
        switch_wifi_network,
        disconnect,
    },
};

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
    match get_current_ssid() {
        Ok(current_ssid) => writeln!(interface, "Current SSID: {}", current_ssid).unwrap(),
        _ =>  writeln!(interface, "Current SSID: <not configured>").unwrap(),
    }
}

fn cmd_network_scan(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    _context: &mut Context,
) {
    let lst = scan_wifi();

    if lst.is_empty() {
        writeln!(interface, "no network found").unwrap();
    }

    writeln!(interface, "network(s) found: {}", lst.len()).unwrap();

    for line in lst {
        writeln!(
            interface,
            "  {}",
            line
        )
        .unwrap();
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

    let _ = switch_wifi_network(args[0], args[1]).or_else(|_| {
        writeln!(interface, "Failed to connect to WiFi network.")
    });

    context.nvs.set_network_configuration(args[0], args[1]).unwrap_or_else(|e| {
        writeln!(interface, "Failed to save network configuration: {:?}", e).unwrap();
    });
}

fn cmd_network_ip(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    _context: &mut Context,
) {
    if let Ok((ip_str, netmask_str, gateway_str)) = interface_str() {
        writeln!(
            interface,
            "IP Address: {}",
            ip_str
        )
        .unwrap();

        writeln!(
            interface,
            "Netmask: {}",
            netmask_str
        )
        .unwrap();

        writeln!(
            interface,
            "Gateway: {}",
            gateway_str
        )
        .unwrap();
    }
}

fn cmd_network_disconnect(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    interface: &mut UartIo,
    context: &mut Context,
) {
    match disconnect() {
        Ok(_) => {
            writeln!(interface, "Disconnected from WiFi successfully!").unwrap();
            context.nvs.clear_network_configuration().unwrap_or_else(|e| {
                writeln!(interface, "Failed to clear network configuration: {:?}", e).unwrap();
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
    _context: &mut Context,
) {
    if args.is_empty() {
        if let Ok(hostname) = get_hostname() {
            writeln!(interface, "Current hostname: {}", hostname).unwrap();
        } else {
            writeln!(interface, "Error retrieving hostname").unwrap();
        }
    } else {
        // if let Some(mutex_wifi) = WIFI.get() {
            //     let mut wifi = mutex_wifi.lock().unwrap();
            //     let value_str = match menu::argument_finder(item, args, "value") {
            //         Ok(Some(v)) => {
            //             wifi.disconnect()?;
            //             wifi.wifi_mut().sta_netif_mut().set_hostname(value_str)?;
            //         },
            //         _ => {
                        
            //         }
            //     };
            // }
    }
}

