use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    prelude::*,
};

use super::AppModel;

relm4::new_action_group!(pub(crate) AppActions, "app");

relm4::new_stateless_action!(pub(crate) ShowPreferences, AppActions, "preferences");
relm4::new_stateless_action!(pub(crate) ShowKeyboardShortcuts, AppActions, "shortcuts");
relm4::new_stateless_action!(pub(crate) ShowHelp, AppActions, "help");
relm4::new_stateless_action!(pub(crate) ShowAbout, AppActions, "about");

impl AppModel {
    pub(crate) fn create_actions(
        widgets: &<Self as SimpleComponent>::Widgets,
        sender: &ComponentSender<Self>,
    ) {
        let app = relm4::main_adw_application();

        let mut app_actions = RelmActionGroup::<AppActions>::new();

        let show_preferences = {
            let sender = sender.clone();
            RelmAction::<ShowPreferences>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowPreferencesWindow);
            })
        };
        app_actions.add_action(show_preferences);

        let show_keyboard_shortcuts = {
            let sender = sender.clone();
            RelmAction::<ShowKeyboardShortcuts>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowKeyboardShortcutsWindow);
            })
        };
        app_actions.add_action(show_keyboard_shortcuts);

        let show_help = {
            let sender = sender.clone();
            RelmAction::<ShowHelp>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowHelpWindow);
            })
        };
        app.set_accelerators_for_action::<ShowHelp>(&["F1"]);
        app_actions.add_action(show_help);

        let show_about = {
            let sender = sender.clone();
            RelmAction::<ShowAbout>::new_stateless(move |_| {
                sender.input(<Self as SimpleComponent>::Input::ShowAboutWindow);
            })
        };
        app_actions.add_action(show_about);

        app_actions.register_for_widget(&widgets.main_window);
    }
}
