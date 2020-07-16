use cairo::Context;
use gtk::prelude::*;
use gtk::{DrawingArea, Widget};
use lib::{Action, Color, Die, Game, Pos};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

const SQUARE_SIZE: f64 = 60.0;
const SIZE: f64 = SQUARE_SIZE * 12.0;
const DIE_SIZE: f64 = SQUARE_SIZE - 4.0;
const ORIGIN_X: f64 = SIZE / 2.0 - DIE_SIZE / 2.0;
const ORIGIN_Y: f64 = SIZE / 2.0 + 2.0;

const BACKGROUND_COLOR: CairoColor = CairoColor::RGB(0.0, 1.0 / 3.0, 0.0);
const RED_COLOR: CairoColor = CairoColor::RGB(1.0, 0.0, 0.0);
const BLACK_COLOR: CairoColor = CairoColor::RGB(0.0, 0.0, 0.0);
const DOT_COLOR: CairoColor = CairoColor::RGB(1.0, 1.0, 1.0);
const HIGHLIGHT_EMPTY_POS_COLOR: CairoColor = CairoColor::RGBA(0.0, 0.0, 0.0, 0.4);

pub struct GameCanvas {
    game: Rc<RefCell<Game>>,
    canvas: DrawingArea,
}

impl Default for GameCanvas {
    fn default() -> Self {
        Self::new()
    }
}

impl GameCanvas {
    pub fn new() -> Self {
        Self {
            game: Rc::new(RefCell::new(Game::new())),
            canvas: DrawingArea::new(),
        }
        .init()
    }

    fn init(self) -> Self {
        self.canvas.set_size_request(SIZE as i32, SIZE as i32);
        let game = Rc::clone(&self.game);
        self.canvas.connect_draw(move |_, context| {
            let game = game.borrow();
            set_color(context, BACKGROUND_COLOR);
            context.paint();
            for (&pos, &die) in game.board().iter() {
                draw_die(context, pos, die);
            }
            for action in game.actions() {
                match action {
                    &Action::Add(pos) => {
                        highlight_empty_pos(context, pos);
                    }
                    Action::Merge(_, _) => {}
                    Action::Move(_, _) => {}
                }
            }
            Inhibit(false)
        });
        self
    }

    pub fn widget(&self) -> &impl IsA<Widget> {
        &self.canvas
    }
}

fn draw_die(context: &Context, Pos(x, y): Pos, die: Die) {
    match die.color() {
        Color::Red => set_color(context, RED_COLOR),
        _ => set_color(context, BLACK_COLOR),
    }
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    context.rectangle(x, y, DIE_SIZE, DIE_SIZE);
    context.fill();
    draw_dots(context, x, y, die);
}

fn highlight_empty_pos(context: &Context, Pos(x, y): Pos) {
    set_color(context, HIGHLIGHT_EMPTY_POS_COLOR);
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    context.rectangle(x, y, DIE_SIZE, DIE_SIZE);
    context.fill();
}

fn draw_dots(context: &Context, x: f64, y: f64, die: Die) {
    set_color(context, DOT_COLOR);
    match die.value() {
        1 => draw_dot(context, x, y, 0, 0),
        2 => {
            draw_dot(context, x, y, -1, 1);
            draw_dot(context, x, y, 1, -1);
        }
        3 => {
            draw_dot(context, x, y, 0, 0);
            draw_dot(context, x, y, -1, 1);
            draw_dot(context, x, y, 1, -1);
        }
        4 => {
            draw_dot(context, x, y, -1, 1);
            draw_dot(context, x, y, -1, -1);
            draw_dot(context, x, y, 1, 1);
            draw_dot(context, x, y, 1, -1);
        }
        5 => {
            draw_dot(context, x, y, 0, 0);
            draw_dot(context, x, y, -1, 1);
            draw_dot(context, x, y, -1, -1);
            draw_dot(context, x, y, 1, 1);
            draw_dot(context, x, y, 1, -1);
        }
        6 => {
            draw_dot(context, x, y, -1, 1);
            draw_dot(context, x, y, -1, -1);
            draw_dot(context, x, y, 1, 1);
            draw_dot(context, x, y, 1, -1);
            draw_dot(context, x, y, -1, 0);
            draw_dot(context, x, y, 1, 0);
        }
        _ => {}
    }
}

fn draw_dot(context: &Context, x: f64, y: f64, xalign: i32, yalign: i32) {
    let xc = x + DIE_SIZE / 2.0 + xalign as f64 * DIE_SIZE / 4.0;
    let yc = y + DIE_SIZE / 2.0 + yalign as f64 * DIE_SIZE / 4.0;
    context.arc(xc, yc, DIE_SIZE / 10.0, 0.0, 2.0 * PI);
    context.fill();
}

enum CairoColor {
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
}

fn set_color(context: &Context, color: CairoColor) {
    match color {
        CairoColor::RGB(r, g, b) => context.set_source_rgb(r, g, b),
        CairoColor::RGBA(r, g, b, a) => context.set_source_rgba(r, g, b, a),
    }
}