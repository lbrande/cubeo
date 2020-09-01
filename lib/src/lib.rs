use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Not,
    vec::IntoIter,
};

const  MAX_NDICE: usize = 6;
const MAX_VALUE: usize = 6;

#[derive(Clone, Debug)]
pub  struct Game {
    board: HashMap<Pos, Die>,
    ndice: [usize; 2],
    actions: HashSet<Action>,
    turn: Color,
    winner: Option<Color>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: HashMap::with_capacity(MAX_NDICE * 2),
            ndice: [1; 2],
            actions: HashSet::new(),
            turn: Color::Red,
            winner: None,
        }
        .init()
    }

    fn init(mut self) -> Self {
        self.board.insert(Pos(0, 1), Die::with_color(Color::Black));
        self.board.insert(Pos(0, 0), Die::with_color(Color::Red));
        self.update_actions();
        self
    }

    pub fn perform_action(&mut self, action: Action) -> Option<Color> {
        self.winner = action.perform(self).or_else(|| {
            self.turn = !self.turn;
            self.update_actions();
            Some(!self.turn).filter(|_| self.actions.is_empty())
        });
        self.winner
    }

    pub fn board(&self) -> &HashMap<Pos, Die> {
        &self.board
    }

    pub fn actions(&self) -> &HashSet<Action> {
        &self.actions
    }

    pub fn turn(&self) -> Color {
        self.turn
    }

    pub fn winner(&self) -> Option<Color> {
        self.winner
    }

    fn update_actions(&mut self) {
        self.actions.clear();
        self.update_add_actions();
        self.update_merge_actions();
        self.update_move_actions();
    }

    fn update_add_actions(&mut self) {
        if self.ndice[usize::from(self.turn)] < MAX_NDICE {
            for (pos, die) in self.board.iter() {
                if die.color == self.turn {
                    for pos in pos.adjacents() {
                        if self.is_empty(pos)
                            && !pos.adjacents().any(|pos| self.has_color(pos, !die.color))
                        {
                            self.actions.insert(Action::Add(pos));
                        }
                    }
                }
            }
        }
    }

    fn update_merge_actions(&mut self) {
        for (&from, die) in self.board.iter() {
            if die.color == self.turn && self.is_free(from) {
                for to in from.adjacents() {
                    if self.has_color(to, die.color) {
                        self.actions.insert(Action::Merge(from, to));
                    }
                }
            }
        }
    }

    fn update_move_actions(&mut self) {
        for (&from, die) in self.board.iter() {
            if die.color == self.turn && self.is_free(from) {
                let mut tos = HashSet::new();
                tos.insert(from);
                for _ in 0..die.value {
                    tos = tos.iter().flat_map(|&pos| self.steps(pos, from)).collect();
                }
                tos.remove(&from);
                for to in tos {
                    self.actions.insert(Action::Move(from, to));
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

    fn steps(&self, Pos(x, y): Pos, moved_pos: Pos) -> IntoIter<Pos> {
        let mut steps = Vec::with_capacity(4);
        let is_empty = |pos| pos == moved_pos || self.is_empty(pos);
        for &d_x in &[-1i32, 0, 1] {
            for &d_y in &[-1i32, 0, 1] {
                let pos = Pos(x + d_x, y + d_y);
                if d_x.abs() + d_y.abs() == 2 {
                    let s1 = Pos(x + d_x, y);
                    let s2 = Pos(x, y + d_y);
                    if is_empty(pos) && is_empty(s1) ^ is_empty(s2) {
                        steps.push(pos);
                    }
                } else if d_x != 0 {
                    let c1 = Pos(x + d_x, y - 1);
                    let s1 = Pos(x, y - 1);
                    let c2 = Pos(x + d_x, y + 1);
                    let s2 = Pos(x, y + 1);
                    if is_empty(pos)
                        && ((!is_empty(c1) && !is_empty(s1)) || (!is_empty(c2) && !is_empty(s2)))
                    {
                        steps.push(pos);
                    }
                } else if d_y != 0 {
                        let c1 = Pos(x - 1, y + d_y);
                        let s1 = Pos(x - 1, y);
                        let c2 = Pos(x + 1, y + d_y);
                        let s2 = Pos(x + 1, y);
                    if is_empty(pos)
                        && ((!is_empty(c1) && !is_empty(s1)) || (!is_empty(c2) && !is_empty(s2)))
                    {
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

    fn has_color(&self, pos: Pos, color: Color) -> bool {
        self.board.get(&pos).filter(|die| die.color == color).is_some()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Action {
    Add(Pos),
    Merge(Pos, Pos),
    Move(Pos, Pos),
}

impl Action {
    pub fn from(self) -> Option<Pos> {
        match self {
            Self::Merge(from, _) => Some(from),
            Self::Move(from, _) => Some(from),
            _ => None,
        }
    }

    fn perform(self, game: &mut Game) -> Option<Color> {
        match self {
            Self::Add(pos) => {
                game.board.insert(pos, Die::with_color(game.turn));
                game.ndice[usize::from(game.turn)] += 1;
                None
            }
            Self::Merge(from, to) => {
                let from_die = game.board.remove(&from).unwrap();
                let to_die = game.board.get_mut(&to).unwrap();
                to_die.value += from_die.value;
                game.ndice[usize::from(game.turn)] -= 1;
                Some(game.turn).filter(|_| to_die.value > MAX_VALUE)
            }
            Self::Move(from, to) => {
                let die = game.board.remove(&from).unwrap();
                game.board.insert(to, die);
                None
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
        match self {
            Self::Red => Self::Black,
            _ => Self::Red,
        }
    }
}

impl From<Color> for usize {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => 0,
            _ => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Pos(pub i32, pub i32);

impl Pos {
    fn adjacents(self) -> IntoIter<Pos> {
        let Pos(x, y) = self;
        vec![Pos(x - 1, y), Pos(x, y - 1), Pos(x, y + 1), Pos(x + 1, y)].into_iter()
    }
}
