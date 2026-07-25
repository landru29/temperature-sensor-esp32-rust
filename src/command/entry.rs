use menu::{
    Item,
    ItemType,
    Menu,
};

use super::temperature::TEMPERATURE_MENU;
use super::network::NETWORK_MENU;
use super::clock::CLOCK_MENU;
use crate::application::uart::UartIo;
use crate::application::context::Context;


pub const ROOT_MENU: Menu<UartIo, Context> = Menu {
    label: "root",
    items: &[
        &Item {
            item_type: ItemType::Menu(&NETWORK_MENU),
            command: "network",
            help: Some("network related commands"),
        },
        &Item {
            item_type: ItemType::Menu(&TEMPERATURE_MENU),
            command: "temperature",
            help: Some("temperature related commands"),
        },
        &Item {
            item_type: ItemType::Menu(&CLOCK_MENU),
            command: "clock",
            help: Some("clock related commands"),
        },
    ],
    entry: None,
    exit: None,
};