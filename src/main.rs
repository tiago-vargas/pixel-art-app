use relm4::prelude::*;

mod app;

fn main() {
    let app = RelmApp::new(app::APP_ID);
    app.run::<app::AppModel>(());
}
