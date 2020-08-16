use cairo::Context;
use gdk::EventMask;
use gtk::prelude::*;
use gtk::{DrawingArea, Widget};
use lib::{Action, Color, Die, Game, Pos};
use std::{cell::RefCell, f64::consts::PI, rc::Rc};

const SQUARE_SIZE: f64 = 60.0;
const SIZE: f64 = SQUARE_SIZE * 12.0;
const DIE_SIZE: f64 = SQUARE_SIZE - 4.0;
const CORNER_SIZE: f64 = 6.0;
const ORIGIN_X: f64 = SIZE / 2.0 - DIE_SIZE / 2.0;
const ORIGIN_Y: f64 = SIZE / 2.0 + 2.0;

const BACKGROUND_COLOR: CairoColor = CairoColor::RGB(0.15, 0.35, 0.15);
const RED_COLOR: CairoColor = CairoColor::RGB(0.55, 0.15, 0.15);
const BLACK_COLOR: CairoColor = CairoColor::RGB(0.15, 0.15, 0.15);
const DOT_COLOR: CairoColor = CairoColor::RGB(0.85, 0.85, 0.85);
const BORDER_COLOR: CairoColor = CairoColor::RGBA(1.0, 1.0, 1.0, 0.2);
const TARGET_COLOR: CairoColor = CairoColor::RGBA(0.0, 0.0, 0.0, 0.4);
const TARGET_DIE_COLOR: CairoColor = CairoColor::RGB(0.85, 0.85, 0.85);
const SELECTED_DIE_COLOR: CairoColor = CairoColor::RGB(0.85, 0.85, 0.15);

pub struct GameCanvas {
    data: Rc<RefCell<GameCanvasData>>,
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
            data: Rc::new(RefCell::new(GameCanvasData { game: Game::new(), selected_pos: None })),
            canvas: DrawingArea::new(),
        }
        .init()
    }

    fn init(mut self) -> Self {
        self.canvas.set_size_request(SIZE as i32, SIZE as i32);
        self.init_draw();
        self.init_event();
        self
    }

    fn init_draw(&mut self) {
        let data = Rc::clone(&self.data);
        self.canvas.connect_draw(move |_, context| {
            let data = data.borrow();
            set_color(context, BACKGROUND_COLOR);
            context.paint();
            for (&pos, &die) in data.game.board().iter() {
                draw_die(context, pos, die);
            }
            if data.game.winner().is_none() {
                if let Some(pos) = data.selected_pos {
                    target_die(context, pos, SELECTED_DIE_COLOR);
                    for action in data.game.actions() {
                        if action.from() == Some(pos) {
                            match action {
                                Action::Merge(_, to) => target_die(context, *to, TARGET_DIE_COLOR),
                                Action::Move(_, to) => target_pos(context, *to, TARGET_COLOR),
                                _ => {}
                            }
                        }
                    }
                } else {
                    for action in data.game.actions() {
                        match action {
                            Action::Add(pos) => target_pos(context, *pos, TARGET_COLOR),
                            Action::Merge(from, _) => target_die(context, *from, TARGET_DIE_COLOR),
                            Action::Move(from, _) => target_die(context, *from, TARGET_DIE_COLOR),
                        }
                    }
                }
            }
            Inhibit(false)
        });
    }

    fn init_event(&mut self) {
        let data = Rc::clone(&self.data);
        self.canvas.connect_button_press_event(move |canvas, event| {
            let mut data = data.borrow_mut();
            if data.game.winner().is_none() {
                let x = ((event.get_position().0 - ORIGIN_X) / SQUARE_SIZE).floor() as i32;
                let y = ((ORIGIN_Y - event.get_position().1) / SQUARE_SIZE).ceil() as i32;
                let pos = Pos(x, y);
                if let Some(from) = data.selected_pos {
                    if let Some(&action) = (data.game.actions().get(&Action::Merge(from, pos)))
                        .or_else(|| data.game.actions().get(&Action::Move(from, pos)))
                    {
                        data.game.perform_action(action);
                    }
                    data.selected_pos = None;
                } else if let Some(&action) = data.game.actions().get(&Action::Add(pos)) {
                    data.game.perform_action(action);
                    data.selected_pos = None;
                } else if (data.game.actions().iter()).any(|action| action.from() == Some(pos)) {
                    data.selected_pos = Some(pos)
                }
                canvas.queue_draw();
            }
            Inhibit(false)
        });
        self.canvas.add_events(EventMask::BUTTON_PRESS_MASK);
    }

    pub fn widget(&self) -> &impl IsA<Widget> {
        &self.canvas
    }

    pub fn data(&self) -> Rc<RefCell<GameCanvasData>> {
        Rc::clone(&self.data)
    }
}

pub struct GameCanvasData {
    game: Game,
    selected_pos: Option<Pos>,
}

impl GameCanvasData {
    pub fn game(&self) -> &Game {
        &self.game
    }
}

fn draw_die(context: &Context, Pos(x, y): Pos, die: Die) {
    match die.color() {
        Color::Red => set_color(context, RED_COLOR),
        _ => set_color(context, BLACK_COLOR),
    }
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    die_rectangle(context, x, y);
    context.fill_preserve();
    set_color(context, BORDER_COLOR);
    context.stroke();
    draw_dots(context, x, y, die);
}

fn target_pos(context: &Context, Pos(x, y): Pos, color: CairoColor) {
    set_color(context, color);
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    die_rectangle(context, x, y);
    context.fill();
}

fn target_die(context: &Context, Pos(x, y): Pos, color: CairoColor) {
    set_color(context, color);
    let x = ORIGIN_X + x as f64 * SQUARE_SIZE;
    let y = ORIGIN_Y - y as f64 * SQUARE_SIZE;
    die_rectangle(context, x, y);
    context.stroke();
}

fn die_rectangle(context: &Context, x: f64, y: f64) {
    let xleft = x + CORNER_SIZE;
    let xright = x + DIE_SIZE - CORNER_SIZE;
    let ytop = y + CORNER_SIZE;
    let ybottom = y + DIE_SIZE - CORNER_SIZE;
    context.new_sub_path();
    context.arc(xleft, ytop, CORNER_SIZE, PI, 1.5 * PI);
    context.arc(xright, ytop, CORNER_SIZE, 1.5 * PI, 0.0);
    context.arc(xright, ybottom, CORNER_SIZE, 0.0, 0.5 * PI);
    context.arc(xleft, ybottom, CORNER_SIZE, 0.5 * PI, PI);
    context.close_path();
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
        _ => {
            let xc = x + DIE_SIZE / 2.0;
            let yc = y + DIE_SIZE / 2.0;
            context.arc(xc, yc, DIE_SIZE / 2.5, 0.0, 2.0 * PI);
            context.fill();
        }
    }
}

fn draw_dot(context: &Context, x: f64, y: f64, xalign: i32, yalign: i32) {
    let xc = x + DIE_SIZE / 2.0 + xalign as f64 * DIE_SIZE / 4.0;
    let yc = y + DIE_SIZE / 2.0 + yalign as f64 * DIE_SIZE / 4.0;
    context.arc(xc, yc, DIE_SIZE / 10.0, 0.0, 2.0 * PI);
    context.fill();
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
