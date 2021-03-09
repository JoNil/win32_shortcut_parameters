#![windows_subsystem = "windows"]

use msgbox::IconType;

fn main() {
    msgbox::create("Hello Title", "Hello World!", IconType::Info).ok();
}
