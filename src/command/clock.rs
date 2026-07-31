use menu::*;
// use embedded_io::Write;

use crate::application::{
    uart::UartIo,
    context::{
        Context,
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
    ],
    entry: None,
    exit: None,
};


fn cmd_clock_start(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    _interface: &mut UartIo,
    context: &mut Context,
) {
    // match context.timer.start() {
    //     Ok(_) => {
    //         writeln!(_interface, "Clock started.").unwrap();
    //     }
    //     Err(e) => {
    //         writeln!(_interface, "Failed to start clock: {:?}", e).unwrap();
    //     }
    // };
}

fn cmd_clock_stop(
    _menu: &Menu<UartIo, Context>,
    _item: &Item<UartIo, Context>,
    _args: &[&str],
    _interface: &mut UartIo,
    context: &mut Context,
) {
    // match context.timer.stop() {
    //     Ok(_) => {
    //         writeln!(_interface, "Clock stopped: {:?}.", context.timer.get_counter()).unwrap();
    //     }
    //     Err(e) => {
    //         writeln!(_interface, "Failed to stop clock: {:?}", e).unwrap();
    //     }
    // }
}

