//#![windows_subsystem = "windows"]

use log::LevelFilter;
use msgbox::IconType;
use std::env;
use win32_shortcut_parameters::{get_shortcut_args, set_shortcut_args};

fn main() {
    notepad_logger::init().unwrap();
    log::set_max_level(LevelFilter::Trace);

    let args = env::args().skip(1).collect::<Vec<_>>().join(" ");

    let shortcut_args = get_shortcut_args();

    let message = format!("Hello World! {} {:?}", args, &shortcut_args);

    if let Ok(args) = shortcut_args {
        if let Err(e) = set_shortcut_args(&format!("{} -test2", args)) {
            msgbox::create("Hello Title", &format!("{:?}", e), IconType::Info).ok();
        }
    }

    msgbox::create("Hello Title", &message, IconType::Info).ok();
}
