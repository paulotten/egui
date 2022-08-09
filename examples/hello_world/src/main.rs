#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// Linux, MacOS, Windows entry point
fn main() {
    hello_world::run()
}
