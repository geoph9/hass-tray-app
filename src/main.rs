mod config;

use tray_item::TrayItem;
use gtk;
use std::path::Path;
use config::{Config, read_config};

fn main() {

    let current_dir = env!("CARGO_MANIFEST_DIR");
    let icons_dir = Path::new(current_dir).join("icons");
    let config_dir = Path::new(current_dir).join("config");

    let settings: Config = read_config(
        config_dir.
            join("config.toml").
            to_str().
            expect("Something happened")
    );

    println!("weather_settings->refresh_rate: {:?}", settings.weather_settings.refresh_rate);

    gtk::init().unwrap();

    let mut tray = TrayItem::new("Tray Example", "").unwrap();
    tray.set_icon(icons_dir.join("hass-main.png").to_str().unwrap()).unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", || {
        println!("Hello!");
    }).unwrap();

    // Separator
    tray.add_label("").unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    gtk::main();

}
