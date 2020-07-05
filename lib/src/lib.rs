use std::{
    collections::{HashMap, HashSet, VecDeque},
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
        let mut board = HashMap::with_capacity(NDICE * 2);
        board.insert(Pos(0, 0), Die::with_color(Color::Red));
        board.insert(Pos(0, 1), Die::with_color(Color::Black));
        Self {
            board,
            turn: Color::Red,
        }
    }

    pub fn perform_action(&mut self, action: Action) {
        action.perform(self);
        self.turn = !self.turn;
    }

    pub fn actions(&self) -> HashSet<Action> {
        let mut actions = HashSet::new();
        self.insert_add_actions(&mut actions);
        self.insert_merge_actions(&mut actions);
        self.insert_move_actions(&mut actions);
        actions
    }

    fn insert_add_actions(&self, actions: &mut HashSet<Action>) {
        for (pos, die) in self.board.iter() {
            if die.color == self.turn {
                for pos in pos.adjacents() {
                    if self.is_empty(pos)
                        && !pos.adjacents().any(|pos| self.has_color(!die.color, pos))
                    {
                        actions.insert(Action::Add(pos, self.turn));
                    }
                }
            }
        }
    }

    fn insert_merge_actions(&self, actions: &mut HashSet<Action>) {
        for (&from, die) in self.board.iter() {
            if die.color == self.turn && self.is_free(from) {
                for to in from.adjacents() {
                    if self.has_color(die.color, to) {
                        actions.insert(Action::Merge(from, to));
                    }
                }
            }
        }
    }

    fn insert_move_actions(&self, actions: &mut HashSet<Action>) {
        for (&from, die) in self.board.iter() {
            if die.color == self.turn && self.is_free(from) {
                let mut tos = HashSet::with_capacity(die.value * 4);
                tos.insert(from);
                for _ in 0..die.value {
                    let old_tos = tos.clone().into_iter();
                    tos.clear();
                    for old_to in old_tos {
                        for to in self.steps(old_to) {
                            tos.insert(to);
                        }
                    }
                }
                tos.remove(&from);
                for to in tos {
                    actions.insert(Action::Move(from, to));
                }
            }
        }
    }

    fn is_free(&self, pos: Pos) -> bool {
        let mut queue = VecDeque::new();
        queue.push_back(pos.adjacents().find(|&pos| !self.is_empty(pos)).unwrap());
        let mut visited = HashSet::with_capacity(self.board.len());
        visited.insert(pos);
        while let Some(node) = queue.pop_front() {
            visited.insert(node);
            for node in node.adjacents() {
                if !visited.contains(&node) && !self.is_empty(node) {
                    queue.push_back(node);
                }
            }
        }
        visited.len() == self.board.len()
    }

    fn steps(&self, from: Pos) -> IntoIter<Pos> {
        let Pos(x, y) = from;
        let mut steps = Vec::with_capacity(4);
        for &d_x in &[-1i32, 0, 1] {
            for &d_y in &[-1i32, 0, 1] {
                if d_x.abs() + d_y.abs() == 2 {
                    // diagonals
                    let pos = Pos(x + d_x, y + d_y);
                    let a_pos = Pos(x + d_x, y);
                    let b_pos = Pos(x, y + d_y);
                    if self.is_empty(pos)
                        && ((self.is_empty(a_pos) && !self.is_empty(b_pos))
                            || (!self.is_empty(a_pos) && self.is_empty(b_pos)))
                    {
                        steps.push(pos);
                    }
                } else if d_x != 0 {
                    // horixontal orthogonals
                    let pos = Pos(x + d_x, y);
                    let a_pos = Pos(x + d_x, y - 1);
                    let b_pos = Pos(x + d_x, y + 1);
                    if self.is_empty(pos) && (!self.is_empty(a_pos) || !self.is_empty(b_pos)) {
                        steps.push(pos);
                    }
                } else if d_y != 0 {
                    // vertical orthogonals
                    let pos = Pos(x, y + d_y);
                    let a_pos = Pos(x - 1, y + d_y);
                    let b_pos = Pos(x + 1, y + d_y);
                    if self.is_empty(pos) && (!self.is_empty(a_pos) || !self.is_empty(b_pos)) {
                        steps.push(pos);
                    }
                }
            }
        }
        steps.into_iter()
    }

    fn is_empty(&self, pos: Pos) -> bool {
        !self.board.contains_key(&pos)
    }

    fn has_color(&self, color: Color, pos: Pos) -> bool {
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
            Self::Add(pos, color) => {
                game.board.insert(pos, Die::with_color(color));
            }
            Self::Merge(from, to) => {
                let from_die = game.board.remove(&from).unwrap();
                game.board
                    .entry(to)
                    .and_modify(|die| die.value += from_die.value);
            }
            Self::Move(from, to) => {
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
    pub fn with_color(color: Color) -> Self {
        Self { color, value: 1 }
    }

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
pub struct Pos(i32, i32);

impl Pos {
    fn adjacents(self) -> IntoIter<Pos> {
        let Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x, y - 1), Pos(x, y + 1), Pos(x + 1, y)].into_iter()
    }
}
