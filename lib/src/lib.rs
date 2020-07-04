use std::collections::HashMap;

const NDICE: usize = 6;

#[derive(Clone, Debug)]
pub struct Game {
    board: HashMap<Pos, Die>,
    turn: Color,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: HashMap::with_capacity(NDICE * 2),
            turn: Color::Red,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct AddAction {
    pos: Pos,
    color: Color,
}

impl Action for AddAction {
    fn perform(self, game: &mut Game) {
        let die = Die {
            color: self.color,
            value: 1,
        };
        game.board.insert(self.pos, die);
    }
}

impl AddAction {
    pub fn pos(self) -> Pos {
        self.pos
    }

    pub fn color(self) -> Color {
        self.color
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MergeAction {
    from: Pos,
    to: Pos,
}

impl Action for MergeAction {
    fn perform(self, game: &mut Game) {
        let from_die = game.board.remove(&self.from).unwrap();
        game.board
            .entry(self.to)
            .and_modify(|e| e.value += from_die.value);
    }
}

impl MergeAction {
    pub fn from(self) -> Pos {
        self.from
    }

    pub fn to(self) -> Pos {
        self.to
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MoveAction {
    from: Pos,
    to: Pos,
}

impl Action for MoveAction {
    fn perform(self, game: &mut Game) {
        let die = game.board.remove(&self.from).unwrap();
        game.board.insert(self.to, die);
    }
}

impl MoveAction {
    pub fn from(self) -> Pos {
        self.from
    }

    pub fn to(self) -> Pos {
        self.to
    }
}

trait Action {
    fn perform(self, game: &mut Game);
}

#[derive(Copy, Clone, Debug)]
pub struct Die {
    color: Color,
    value: usize,
}

impl Die {
    pub fn color(self) -> Color {
        self.color
    }

    pub fn value(self) -> usize {
        self.value
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Red,
    Black,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos(usize, usize);
