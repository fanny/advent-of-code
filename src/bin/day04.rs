use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Board {
    nums: HashSet<u32>,
    rows: HashMap<i32, Vec<u32>>,
}

type Boards = HashMap<usize, Board>;

fn parse_body(lines: &[&str]) -> Board {
    lines.into_iter().enumerate().fold(
        Board {
            nums: HashSet::new(),
            rows: HashMap::new(),
        },
        |mut board, (row, values)| {
            values
                .trim()
                .split_whitespace()
                .enumerate()
                .for_each(|(column, value)| {
                    let parsed_col = column as i32;
                    let parsed_row = row as i32;
                    let v = value.parse::<u32>().unwrap();

                    board.nums.insert(v);
                    board.rows.entry(-(parsed_col + 1)).or_default().push(v);
                    board.rows.entry(parsed_row + 1).or_default().push(v);
                });
            board
        },
    )
}

fn find_winners(boards: &Boards, parsed_guesses: &[u32], cur_index: usize) -> Vec<(usize, u32)> {
    let current_guess = &parsed_guesses[cur_index - 1];
    let current_guesses = &parsed_guesses[..cur_index];

    boards
        .into_iter()
        .filter(|(_key, b)| {
            b.nums.contains(current_guess)
                && b.rows
                    .values()
                    .any(|v| v.into_iter().all(|n| current_guesses.contains(n)))
        })
        .map(|(key, b)| {
            let uncrossed_numbers_sum: u32 =
                b.nums.iter().filter(|n| !current_guesses.contains(n)).sum();
            (*key, uncrossed_numbers_sum * current_guess)
        })
        .collect::<Vec<(usize, u32)>>()
}

fn find_first_winner(boards: &Boards, guesses: &[u32], i: usize) -> u32 {
    let winners = find_winners(boards, guesses, i);

    if !winners.is_empty() {
        let (_key, score) = winners.first().unwrap();
        *score
    } else {
        find_first_winner(boards, guesses, i + 1)
    }
}

fn find_last_winner(boards: &mut Boards, guesses: &[u32]) -> u32 {
    let mut winners: Vec<u32> = vec![];

    (5..guesses.len()).for_each(|i| {
        find_winners(boards, guesses, i)
            .iter()
            .for_each(|(key, score)| {
                boards.remove(key);
                winners.push(*score);
            })
    });

    *winners.last().unwrap()
}

fn parse_file(filename: &str) -> (Vec<u32>, Boards) {
    let input = fs::read_to_string(filename).unwrap();

    let (guesses, boards) = input.split_once("\n\n").unwrap();

    let parsed_guesses = guesses
        .to_string()
        .split(",")
        .into_iter()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let parsed_boards: Boards = boards
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(5)
        .into_iter()
        .map(parse_body)
        .enumerate()
        .fold(HashMap::new(), |mut acc, (key, board)| {
            acc.insert(key, board);
            acc
        });

    (parsed_guesses, parsed_boards)
}

fn main() {
    let (parsed_guesses, mut parsed_boards) = parse_file("input/04.txt");

    println!(
        "{:#?}",
        find_first_winner(&parsed_boards, &parsed_guesses, 5)
    );
    println!(
        "{:#?}",
        find_last_winner(&mut parsed_boards, &parsed_guesses)
    );
}
