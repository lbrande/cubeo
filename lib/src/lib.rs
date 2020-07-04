use std::{
    collections::{HashMap, HashSet},
    ops::Not,
    vec::IntoIter,
};

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

    pub fn actions(&self) -> HashSet<Action> {
        let mut actions = HashSet::new();
        self.insert_add_actions(&mut actions);
        actions
    }

    fn insert_add_actions(&self, actions: &mut HashSet<Action>) {
        for (pos, die) in self.board.iter() {
            if die.color == self.turn {
                for pos in pos.adjacents() {
                    if !self.board.contains_key(&pos)
                        && !pos.adjacents().any(|pos| self.has_color(pos, !die.color))
                    {
                        actions.insert(Action::Add(pos, self.turn));
                    }
                }
            }
        }
    }

    fn has_color(&self, pos: Pos, color: Color) -> bool {
        self.board
            .get(&pos)
            .filter(|die| die.color == color)
            .is_some()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    Add(Pos, Color),
    Merge(Pos, Pos),
    Move(Pos, Pos),
}

impl Action {
    fn perform(self, game: &mut Game) {
        match self {
            Action::Add(pos, color) => {
                let die = Die { color, value: 1 };
                game.board.insert(pos, die);
            }
            Action::Merge(from, to) => {
                let from_die = game.board.remove(&from).unwrap();
                game.board
                    .entry(to)
                    .and_modify(|die| die.value += from_die.value);
            }
            Action::Move(from, to) => {
                let die = game.board.remove(&from).unwrap();
                game.board.insert(to, die);
            }
        }
    }
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

impl Not for Color {
    type Output = Color;
    fn not(self) -> Self::Output {
        if self == Self::Red {
            Self::Black
        } else {
            Self::Red
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos(usize, usize);

impl Pos {
    fn adjacents(self) -> IntoIter<Pos> {
        let Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x, y - 1), Pos(x, y + 1), Pos(x + 1, y)].into_iter()
    }
}
