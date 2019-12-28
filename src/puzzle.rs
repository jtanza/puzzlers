use std::str;
use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

const EMPTY: &str = "--";

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point{x: usize, y: usize}

pub struct Puzzle<'a> {
    placed: HashMap<String, Vec<Point>>,
    board: [[&'a str; 4]; 4],
    dictionary: HashSet<String>,
}

impl<'a> Puzzle<'a> {
    pub fn new(candidates: &'a [String]) -> Self {
        let mut puzzle = Puzzle {placed: HashMap::new(), board: [[EMPTY; 4]; 4], dictionary: HashSet::from_iter(candidates.iter().cloned())};
        puzzle.start(Puzzle::select_any(candidates));
        puzzle
    }

    pub fn update(&mut self, word: &'a str, variant: &'a str, points: Vec<Point>) {
        self.place(variant, points.clone());
        self.placed.insert(String::from(word), points);
    }

    pub fn find_fit(&mut self, candidate: &'a str, variants: &'a [String]) -> Option<(&'a str, Vec<Point>)> {
        let candidate_substrs: HashSet<&str> = HashSet::from_iter(Puzzle::split(candidate));
        for existing in &self.on_board() {
            if candidate == existing {
                continue;
            }
            for intersection in candidate_substrs.intersection(&HashSet::from_iter(Puzzle::split(existing))) {
                for variant in variants.choose_multiple(&mut rand::thread_rng(), variants.len()) {
                    if let Some(l) = self.candidate_location(existing, variant, intersection) {
                        if !self.is_disruptive(variant, &l) {
                            return Some((variant, l));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn select_any(candidates: &'a [String]) -> &str {
        candidates.choose(&mut thread_rng()).unwrap()
    }

    pub fn is_disruptive(&self, cand: &str, coords: &[Point]) -> bool {
        let point_map = Puzzle::point_map(cand, coords);
        let mut row_word = String::from("");
        let mut col_word = String::from("");
        let mut word_len = 0;

        for i in 0..(self.board.len() * self.board.len()) {
            let row = i / self.board.len();
            let col = i % self.board.len();

            row_word = Puzzle::from_point_or_board(point_map.get(&Point{x:row, y:col}), row_word, self.board[row][col]);
            col_word = Puzzle::from_point_or_board(point_map.get(&Point{x:col, y:row}), col_word, self.board[col][row]);

            if word_len == self.board.len() - 1 { // denotes a full scan
                if self.is_breaking(&row_word) || self.is_breaking(&col_word) {
                    return true;
                } else {
                    word_len = 0;
                    row_word = String::from("");
                    col_word = String::from("");
                }
            } else {
                word_len += 1;
            }
        }
        false
    }

    pub fn place(&mut self, word: &'a str, points: Vec<Point>) {
        let mut i = 0;
        for e in Puzzle::split(word) {
            match points.get(i) {
                Some(point) => {
                    self.board[point.x][point.y] = e;
                    i += 1;
                },
                _ => break // guard against mismatch in word/points len
            }
        }
    }

    pub fn not_complete(&self) -> bool {
        for word in self.on_board() {
            if word.len() < 4 { return true; }
        }
        false
    }

    pub fn make_variants(candidate: &str) -> Vec<String> {
        let mut res = vec![candidate.to_string()];
        let split = Puzzle::split(candidate);
        for i in 0..=split.len() {
            res.push(format!("{}{}{}", split[0..i].concat(), EMPTY, split[i..split.len()].concat()));
        }
        res
    }

    fn candidate_location(&self, existing: &str, candidate: &str, intersec: &str) -> Option<Vec<Point>> {
        self.placed.get(existing)?;

        let existing_coords = self.placed.get(existing).unwrap();
        let candidate_len = candidate.len() / 2;
        let existing_start = *existing_coords.first().unwrap();
        let is_x_oriented = existing_start.x == existing_coords.last().unwrap().x;

        let existing_offset = existing.find(intersec).unwrap() / 2;
        let cand_offset = candidate.find(intersec).unwrap() / 2;

        if is_x_oriented {
            if !Puzzle::overflows(&existing_start, existing_offset, cand_offset, candidate_len, true) {
                Some(Puzzle::gen_coords(Point{ x: existing_start.x - cand_offset, y: existing_start.y + existing_offset }, candidate_len, true))
            } else {
                None
            }
        } else if !Puzzle::overflows(&existing_start, existing_offset, cand_offset, candidate_len, false) {
            Some(Puzzle::gen_coords(Point{ x: existing_start.x + existing_offset, y: existing_start.y - cand_offset }, candidate_len, false))
        } else {
            None
        }
    }

    fn on_board(&self) -> Vec<String> {
        let mut res = vec![];
        let mut row_word = String::from("");
        let mut col_word = String::from("");
        let mut word_len = 0;

        for i in 0..(self.board.len() * self.board.len()) {
            let row = i / self.board.len();
            let col = i % self.board.len();

            match self.board[row][col] {
                EMPTY => {},
                _ => row_word.push_str(self.board[row][col])
            }
            match self.board[col][row] {
                EMPTY => {},
                _ => col_word.push_str(self.board[col][row])
            }

            if word_len == self.board.len() - 1 { // denotes a full scan
                res.push(row_word);
                res.push(col_word);
                word_len = 0;
                row_word = String::from("");
                col_word = String::from("");
            } else {
                word_len += 1;
            }
        }
        res
    }

    fn start(&mut self, word: &'a str) {
        let coords = Puzzle::gen_coords(Point{x:1, y:0}, word.len() / 2, false);
        self.placed.insert(word.to_string(), coords.clone());
        self.place(word, coords);
    }

    fn is_breaking(&self, word: &str) -> bool {
        word.len() > 2 && (word.len() > 6 || !self.dictionary.contains(word))
    }

    fn overflows(existing: &Point, existing_offset: usize, cand_offset: usize, cand_len: usize, is_x_oriented: bool) -> bool {
        if is_x_oriented {
            (existing.x as i32) - (cand_offset as i32) < 0 || existing.y + existing_offset > 4 || existing.x + cand_len > 4
        } else {
            (existing.y as i32) - (cand_offset as i32) < 0 || existing.x + existing_offset > 4 || existing.y + cand_len > 4
        }
    }

    fn from_point_or_board(point: Option<&String>, to_update: String, on_board: &str) -> String {
        let mut res = to_update;
        match point {
            Some(v) => if v != EMPTY { res.push_str(v); },
            None => if on_board != EMPTY { res.push_str(on_board); }
        }
        res
    }

    fn gen_coords(start: Point, candidate_len: usize, is_x_oriented: bool) -> Vec<Point> {
        let mut res = vec![start];
        if is_x_oriented {
            for i in 1..start.x + candidate_len {
                res.push(Point{x: start.x + i, y: start.y})
            }
        } else {
            for i in 1..start.y + candidate_len {
                res.push(Point{x: start.x, y: start.y + i})
            }
        }
        res
    }

    fn split(word: &str) -> Vec<&str> {
        word.as_bytes().chunks_exact(2).map(str::from_utf8).collect::<Result<Vec<&str>, _>>().unwrap()
    }

    fn point_map(cand: &str, coords: &[Point]) -> HashMap<Point, String> {
        let mut res = HashMap::new();
        let mut i = 0;
        for e in Puzzle::split(cand) {
            res.insert(coords[i], String::from(e));
            i += 1;
        }
        res
    }
}

impl<'a> fmt::Display for Puzzle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = String::from("");
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                res.push_str(self.board[i][j]);
            }
            res.push_str("\n");
        }
        write!(f, "{}", res)
    }
}