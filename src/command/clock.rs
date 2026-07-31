use menu::*;
use embedded_io::Write;

use crate::application::{
    uart::UartIo,
    context::{
        Context,
    },
    clock::{
        start_timer,
        stop_timer,
        get_timer_counter,
        reset_timer,
    },
};


pub const CLOCK_MENU: Menu<UartIo, Context> = Menu {
    label: "clock",
    items: &[
        &Item {
            item_type: ItemType::Callback {
                function: cmd_clock_start,
                parameters: &[],
            },
            command: "start",
            help: Some("Starts the clock."),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_clock_stop,
                parameters: &[],
            },
            command: "stop",
            help: Some("Stops the clock."),
        },
        &Item {
            item_type: ItemType::Callback {
                function: cmd_clock_reset,
                parameters: &[],
            },
            command: "reset",
            help: Some("Resets the clock."),
        },
    ],
    entry: Some(enter_clock_menu),
    exit: None,
};

fn enter_clock_menu(
    _menu: &Menu<UartIo, Context>,
    interface: &mut UartIo,
    _context: &mut Context,
) {
    match get_timer_counter() {
        Ok(counter) => {
            writeln!(interface, "Current clock counter: {}", counter).unwrap();
        }
        Err(e) => {
            writeln!(interface, "Failed to retrieve clock counter: {:?}", e).unwrap();
        }
    }
}


fn cmd_clock_start(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    _interface: &mut UartIo,
    _context: &mut Context,
) {
    match start_timer() {
        Ok(_) => {
            writeln!(_interface, "Clock started.").unwrap();
        }
        Err(e) => {
            writeln!(_interface, "Failed to start clock: {:?}", e).unwrap();
        }
    };
}

fn cmd_clock_stop(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    _interface: &mut UartIo,
    _context: &mut Context,
) {
    match stop_timer() {
        Ok(_) => {
            writeln!(_interface, "Clock stopped: {:?}.", get_timer_counter().unwrap_or_else(|_| "Error retrieving counter".to_string())).unwrap();
        }
        Err(e) => {
            writeln!(_interface, "Failed to stop clock: {:?}", e).unwrap();
        }
    }
}

fn cmd_clock_reset(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    _interface: &mut UartIo,
    _context: &mut Context,
) {
    match reset_timer() {
        Ok(_) => {
            writeln!(_interface, "Clock reset.").unwrap();
        }
        Err(e) => {
            writeln!(_interface, "Failed to reset clock: {:?}", e).unwrap();
        }
    }
}