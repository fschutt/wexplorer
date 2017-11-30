//! Main dialog window (Limn application)

use limn::glutin;
use limn::app::App;
use limn::prelude::*;
use limn::input::{EscKeyCloseHandler, DebugSettingsHandler};

pub const APP_TITLE: &'static str = "Explorer";
pub const APP_MIN_WIDTH: u32 = 100;
pub const APP_MIN_HEIGHT: u32 = 200;
pub const APP_DEFAULT_WIDTH: u32 = 800;
pub const APP_DEFAULT_HEIGHT: u32 = 600;

// TODO !!!
#[derive(Debug)]
pub enum InitError { }

pub struct MainDialog {
    app: App,
    root: WidgetBuilder,
}

impl MainDialog {

    pub fn new(width: u32, height: u32, initial_title: &'static str)
    -> Result<Self, InitError>
    {
        let window_builder = glutin::WindowBuilder::new()
            .with_title(initial_title)
            .with_dimensions(width, height)
            .with_min_dimensions(200, 60);

        let events_loop = glutin::EventsLoop::new();
        let window = Window::new(window_builder, &events_loop);
        let mut app = App::new(window, events_loop);

        ::style::ribbon_style::add_ribbon_style();

        // Toggles debug bounds drawing on F1 key
        app.add_handler(DebugSettingsHandler::new());

        let root = WidgetBuilder::new("root");

        Ok(Self {
            app: app,
            root: root,
        })
    }

    /// Example: add a button
    pub fn add_button(&mut self) {
        use limn::widgets::button::ToggleButtonStyle;

        let mut button = ToggleButtonStyle::default();
        button.toggle_text("ON", "OFF");
        let mut button = WidgetBuilder::from_modifier_style(button);
        button.layout().add(constraints![
            center(&self.root),
            bound_by(&self.root).padding(50.0).strength(WEAK),
        ]);
        self.root.add_child(button);
    }

    /// Run the main loop for updating, etc.
    pub fn run_main_loop(self)
    {
        self.app.main_loop(self.root);
    }
}

