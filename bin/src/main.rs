use bin::GameCanvas;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationBuilder, ApplicationWindow};
use lib::Color;
use std::rc::Rc;

fn main() {
    let application = ApplicationBuilder::new().build();
    application.connect_activate(setup);
    application.run(&[]);
}

fn setup(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.set_resizable(false);
    window.set_title("Cubeo");
    let window = Rc::new(window);

    let canvas = GameCanvas::new();
    setup_canvas(Rc::clone(&window), &canvas);
    window.add(canvas.widget());

    window.show_all();
}

fn setup_canvas(window: Rc<ApplicationWindow>, canvas: &GameCanvas) {
    let data = canvas.data();
    canvas.widget().connect_draw(move |_, _| {
        let data = data.borrow();
        match data.game().winner() {
            Some(color) => match color {
                Color::Red => window.set_title("Cubeo - Red Won"),
                _ => window.set_title("Cubeo - Black Won"),
            },
            None => match data.game().turn() {
                Color::Red => window.set_title("Cubeo - Red's Turn"),
                _ => window.set_title("Cubeo - Black's Turn"),
            },
        }
        Inhibit(false)
    });
}
