use std::{collections::HashMap, fs};

struct Point {
    x: u32,
    y: u32,
}

struct Coordinates {
    current: Point,
    target: Point,
}

type Board = HashMap<(u32, u32), u32>;

trait BoardCounter {
    fn add_coordinate(&mut self, line: &str, ignore_diagonals: bool);
    fn overlaps(&self) -> usize;
}

impl Coordinates {
    fn new(line: &str) -> Coordinates {
        let (first, second) = line.split_once(" -> ").unwrap();
        let (x, y) = first.split_once(",").unwrap();
        let (target_x, target_y) = second.split_once(",").unwrap();

        Coordinates {
            current: Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            },
            target: Point {
                x: target_x.parse().unwrap(),
                y: target_y.parse().unwrap(),
            },
        }
    }

    fn are_a_diagonal(&self) -> bool {
        self.current.x != self.target.x && self.current.y != self.target.y
    }

    fn get_new_x(&self, offset: u32) -> u32 {
        if self.current.x < self.target.x {
            self.current.x + offset
        } else if self.current.x > self.target.x {
            self.current.x - offset
        } else {
            self.current.x
        }
    }

    fn get_new_y(&self, offset: u32) -> u32 {
        if self.current.y < self.target.y {
            self.current.y + offset
        } else if self.current.y > self.target.y {
            self.current.y - offset
        } else {
            self.current.y
        }
    }

    fn get_diff(&self) -> u32 {
        let diff_x = self.current.x.abs_diff(self.target.x);
        let diff_y = self.current.y.abs_diff(self.target.y);

        if diff_x > diff_y {
            diff_x
        } else {
            diff_y
        }
    }
}

impl BoardCounter for Board {
    fn add_coordinate(&mut self, line: &str, ignore_diagonals: bool) {
        let coordinates = Coordinates::new(line);

        if ignore_diagonals && coordinates.are_a_diagonal() {
            return;
        }

        (0..(coordinates.get_diff() + 1)).into_iter().for_each(|i| {
            let new_x = coordinates.get_new_x(i);
            let new_y = coordinates.get_new_y(i);

            let counter = self.entry((new_x, new_y)).or_insert(0);
            *counter += 1;
        });
    }

    fn overlaps(&self) -> usize {
        self.values().filter(|v| **v > 1).count()
    }
}

fn solution_b(filename: &str) {
    let input = fs::read_to_string(filename).unwrap();
    let mut board: Board = HashMap::new();
    input
        .lines()
        .for_each(|line| board.add_coordinate(line, true));

    println!("{:#?}", board.overlaps());
}

fn solution_a(filename: &str) {
    let input = fs::read_to_string(filename).unwrap();
    let mut board: Board = HashMap::new();
    input
        .lines()
        .for_each(|line| board.add_coordinate(line, false));

    println!("{:#?}", board.overlaps());
}

fn main() {
    solution_a("input/05.txt");
    solution_b("input/05.txt");
}
