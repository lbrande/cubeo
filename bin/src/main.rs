use bin::GameCanvas;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationBuilder, ApplicationWindow};

fn main() {
    let application = ApplicationBuilder::new().build();
    application.connect_activate(setup);
    application.run(&[]);
}

fn setup(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_resizable(false);
    window.set_title("Cubeo");

    let canvas = GameCanvas::new();
    window.add(canvas.widget());

    window.show_all();
}
