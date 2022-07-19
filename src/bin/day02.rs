use std::fs;

use itertools::Itertools;

struct Position {
    aim: i32,
    horizontal: i32,
    vertical: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            aim: 0,
            horizontal: 0,
            vertical: 0,
        }
    }

    fn parse_command(&self, operator: &str, amount: i32) -> Position {
        match operator {
            "forward" => Position {
                aim: self.aim,
                horizontal: (self.horizontal + self.aim * amount),
                vertical: self.vertical + amount,
            },
            "up" => Position {
                aim: self.aim - amount,
                horizontal: self.horizontal,
                vertical: self.vertical,
            },
            "down" => Position {
                aim: self.aim + amount,
                horizontal: self.horizontal,
                vertical: self.vertical,
            },
            _ => unreachable!(),
        }
    }
}

fn parse_file(filename: &str) -> Position {
    let input = fs::read_to_string(filename).unwrap();

    input.lines().fold(Position::new(), |acc, line| {
        let (operator, amount) = line.split(" ").next_tuple().unwrap();
        acc.parse_command(operator, amount.parse::<i32>().unwrap())
    })
}

fn main() {
    let position = parse_file("input/02.txt");

    println!("Solution: {}", position.horizontal * position.vertical);
}
