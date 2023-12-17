use crate::app;

use relm4::prelude::*;
use gtk::prelude::*;

pub(crate) enum Settings {
    WindowWidth,
    WindowHeight,
    WindowMaximized,
}

impl Settings {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::WindowWidth => "window-width",
            Self::WindowHeight => "window-height",
            Self::WindowMaximized => "window-maximized",
        }
    }
}

impl app::AppModel {
    pub(super) fn save_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(app::APP_ID);

        let (width, height) = widgets.main_window.default_size();
        let _ = settings.set_int(Settings::WindowWidth.as_str(), width);
        let _ = settings.set_int(Settings::WindowHeight.as_str(), height);

        let _ = settings.set_boolean(
            Settings::WindowMaximized.as_str(),
            widgets.main_window.is_maximized(),
        );
    }

    pub(super) fn load_window_state(widgets: &<Self as SimpleComponent>::Widgets) {
        let settings = gtk::gio::Settings::new(app::APP_ID);

        let width = settings.int(Settings::WindowWidth.as_str());
        let height = settings.int(Settings::WindowHeight.as_str());
        widgets.main_window.set_default_size(width, height);

        let maximized = settings.boolean(Settings::WindowMaximized.as_str());
        widgets.main_window.set_maximized(maximized);
    }
}
