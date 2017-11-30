#[macro_use] extern crate failure;
#[macro_use] extern crate limn;
extern crate log;
extern crate hyper;
extern crate hyper_tls;
extern crate ftp;
extern crate chrono;
extern crate dotenv;

/// Module for all non-GUI stuff (core functionality)
pub mod core;
/// Module for all GUI-related stuff (calls into `core` when needed)
pub mod dialogs;
/// Integration with DBus, freedesktop and other applications
pub mod integration;
/// Widgets (custom to this application) + UI styling
pub mod widgets;
/// The core limn-styles for all widgets this application. Can be reused later on.
pub mod style;
/// Unit tests
#[test]
pub mod tests;

pub const APP_TITLE: &str = "Explorer";

fn main() {
    // start the main application
    setup_crash_panic_handler();
    setup_logging();
    parse_cmd_arguments();
    let mut main_dialog = dialogs::main::MainDialog::new(800, 600, APP_TITLE).unwrap();
    // add widgets to the dialog here
    main_dialog.add_button();
    main_dialog.run_main_loop();
}

/// Setup logging and panic failure. On panic, a bug report is
/// automatically submitted
fn setup_logging() {
    ::std::env::set_var("RUST_BACKTRACE", "1");
}

fn setup_crash_panic_handler() {

}

fn parse_cmd_arguments() {

}
