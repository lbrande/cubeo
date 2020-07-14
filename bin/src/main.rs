use cairo::Context;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationBuilder, ApplicationWindow, DrawingArea};
use lib::{Color, Die, Game, Pos};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

const SQUARE_SIZE: f64 = 60.0;
const SIZE: f64 = SQUARE_SIZE * 12.0;
const DIE_SIZE: f64 = SQUARE_SIZE - 4.0;
const ORIGIN_X: f64 = SIZE / 2.0 - DIE_SIZE / 2.0;
const ORIGIN_Y: f64 = SIZE / 2.0 + 2.0;

fn main() {
    let application = ApplicationBuilder::new().build();
    application.connect_activate(setup);
    application.run(&[]);
}

fn setup(app: &Application) {
    let game = Rc::new(RefCell::new(Game::new()));
    let window = ApplicationWindow::new(app);
    window.set_resizable(false);
    window.set_title("Cubeo");

    let canvas = DrawingArea::new();
    setup_canvas(&canvas, Rc::clone(&game));
    window.add(&canvas);

    window.show_all();
}

fn setup_canvas(canvas: &DrawingArea, game: Rc<RefCell<Game>>) {
    canvas.set_size_request(SIZE as i32, SIZE as i32);
    canvas.connect_draw(move |_, context| {
        let game = game.borrow();
        context.set_source_rgb(0.0, 1.0 / 3.0, 0.0);
        context.paint();
        for (&pos, &die) in game.board().iter() {
            draw_die(context, pos, die);
        }
        Inhibit(false)
    });
}

fn draw_die(context: &Context, Pos(x, y): Pos, die: Die) {
    match die.color() {
        Color::Red => context.set_source_rgb(1.0, 0.0, 0.0),
        _ => context.set_source_rgb(0.0, 0.0, 0.0),
    }
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    context.rectangle(x, y, DIE_SIZE, DIE_SIZE);
    context.fill();
    context.set_source_rgb(1.0, 1.0, 1.0);
    let xc = x + DIE_SIZE / 2.0;
    let yc = y + DIE_SIZE / 2.0;
    match die.value() {
        1 => {
            context.arc(xc, yc, DIE_SIZE / 10.0, 0.0, 2.0 * PI);
            context.fill();
        }
        2 => {}
        _ => {}
    }
}
