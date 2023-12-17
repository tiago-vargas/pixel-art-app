use gtk::prelude::*;
use relm4::prelude::*;

mod actions;
mod content;
mod modals;
mod settings;

pub(crate) const APP_ID: &str = "your.app_id";  // TODO: Set app ID

pub(crate) struct AppModel {
    content: Controller<content::ContentModel>,
}

#[derive(Debug)]
pub(crate) enum AppInput {
    ShowPreferencesWindow,
    ShowKeyboardShortcutsWindow,
    ShowHelpWindow,
    ShowAboutWindow,
}

#[derive(Debug)]
pub(crate) enum AppOutput {}

#[relm4::component(pub(crate))]
impl SimpleComponent for AppModel {
    type Init = ();

    type Input = AppInput;
    type Output = AppOutput;

    menu! {
        primary_menu: {
            section! {
                "Preferences" => actions::ShowPreferences,
                "Keyboard Shortcuts" => actions::ShowKeyboardShortcuts,
                "Help" => actions::ShowHelp,
                "About App" => actions::ShowAbout,
            },
        }
    }

    view! {
        main_window = adw::ApplicationWindow {
            set_title: Some("Template"),  // TODO: Set window title
            add_css_class: "devel",

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar {
                    pack_end = &gtk::MenuButton {
                        set_icon_name: "open-menu-symbolic",
                        set_menu_model: Some(&primary_menu),
                    },
                },

                model.content.widget(),
            }
        }
    }

    fn init(
        _init: Self::Init,
        window: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let content = content::ContentModel::builder()
            .launch(content::ContentInit)
            .detach();
        let model = AppModel { content };

        let widgets = view_output!();

        Self::load_window_state(&widgets);
        Self::create_actions(&widgets, &sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        use modals::{about, help, keyboard_shortcuts, preferences};

        match message {
            Self::Input::ShowPreferencesWindow => {
                let app = relm4::main_application();
                let main_window = app.windows().first()
                    .expect("Event should have been triggered by last focused window, thus first item")
                    .clone();

                let preferences_window = preferences::Model::builder()
                    .transient_for(&main_window)
                    .launch(preferences::Init)
                    .detach();

                preferences_window.widget().present();
            }
            Self::Input::ShowKeyboardShortcutsWindow => {
                let keyboard_shortcuts_window = keyboard_shortcuts::Model::builder()
                    .launch(keyboard_shortcuts::Init)
                    .detach();
                keyboard_shortcuts_window.widget().present();
            }
            Self::Input::ShowHelpWindow => {
                let help_window = help::Model::builder()
                    .launch(help::Init)
                    .detach();
                help_window.widget().present();
            }
            Self::Input::ShowAboutWindow => {
                let app = relm4::main_application();
                let main_window = app.windows().first()
                    .expect("Event should have been triggered by last focused window, thus first item")
                    .clone();

                let about_window = about::Model::builder()
                    .transient_for(&main_window)
                    .launch(about::Init)
                    .detach();
                about_window.widget().present();
            }
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        Self::save_window_state(&widgets);
    }
}
