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
        adw::Window {
            set_title: Some("Preferences"),
            set_modal: true,

            adw::HeaderBar,
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
