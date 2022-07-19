use itertools::Itertools;
use std::fs;

fn parse_file(filename: &str) -> Vec<u32> {
    let input = fs::read_to_string(filename).unwrap();

    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn get_increased_count(depths: impl Iterator<Item = u32>) -> usize {
    depths.tuple_windows().filter(|(a, b)| b > a).count()
}

fn get_b_solution(depths: &[u32]) -> usize {
    let depths_windows_sum = depths.iter().tuple_windows().map(|(a, b, c)| a + b + c);

    get_increased_count(depths_windows_sum)
}

fn main() {
    let depths = parse_file("input/01.txt");

    println!(
        "Solution A: {}",
        get_increased_count(depths.iter().cloned())
    );
    println!("Solution B: {}", get_b_solution(&depths));
}
