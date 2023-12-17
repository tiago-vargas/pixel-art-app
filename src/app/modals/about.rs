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
            set_application_name: "Pixel Art App",
            set_developer_name: "Tiago Vargas Pereira de Oliveira",
            set_version: "0.1.0",  // TODO: Set version

            set_website: "https://github.com/tiago-vargas/pixel-art-app",
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
