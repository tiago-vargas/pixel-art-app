use adw::prelude::*;
use relm4::prelude::*;

pub(crate) struct Model;

pub(crate) struct Init;

#[derive(Debug)]
pub(crate) enum Input {}

#[derive(Debug)]
pub(crate) enum Output {}

#[relm4::component(pub(crate))]
impl SimpleComponent for Model {
    type Init = Init;
    type Input = Input;
    type Output = Output;

    view! {
        adw::AboutWindow {
            set_application_icon: "application-x-executable-symbolic",  // TODO: Set app icon
            set_application_name: "Template",  // TODO: Set app name
            set_developer_name: "Someone",  // TODO: Set developer name
            set_version: "0.1.0",  // TODO: Set version

            set_website: "https://github.com/tiago-vargas/relm4-app-template",  // TODO: Set website
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {}
    }
}
