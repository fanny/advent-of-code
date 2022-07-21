use std::fs;

fn binary_arr_to_u32(bits: &[bool]) -> u32 {
    bits.into_iter().fold(0, |acc, &b| acc * 2 + (b as u32))
}

fn get_rating_metric<'a>(
    binary_numbers: &[&'a str],
    rating_filter: impl Fn(Vec<&'a str>, Vec<&'a str>) -> Vec<&'a str>,
) -> u32 {
    let rating_metric = (0..12)
        .into_iter()
        .fold(binary_numbers.to_vec(), |acc, i| {
            if acc.len() == 1 {
                return acc;
            }

            let (contains_one, contains_zero) = acc
                .into_iter()
                .partition(|bits| bits.chars().nth(i).unwrap() == '1');
            rating_filter(contains_one, contains_zero)
        })
        .pop()
        .unwrap();

    u32::from_str_radix(rating_metric, 2).unwrap()
}

fn parse_file(filename: &str) -> Vec<String> {
    let input = fs::read_to_string(filename).unwrap();

    input.lines().map(str::to_string).collect()
}

fn solution_b(binary_numbers: &Vec<String>) -> u32 {
    let binary_numbers_ref = &binary_numbers
        .into_iter()
        .map(String::as_str)
        .collect::<Vec<&str>>();

    let oxygen_rating =
        get_rating_metric(
            binary_numbers_ref,
            |a, b| if a.len() >= b.len() { a } else { b },
        );

    let co2_rating = get_rating_metric(
        binary_numbers_ref,
        |a, b| if a.len() >= b.len() { b } else { a },
    );

    co2_rating * oxygen_rating
}

fn solution_a(binary_numbers: &Vec<String>) -> u32 {
    let most_common_bits = binary_numbers
        .into_iter()
        .fold(vec![0; 12], |mut acc, line| {
            line.chars()
                .enumerate()
                .for_each(|(i, c)| acc[i] += if c == '1' { 1 } else { -1 });
            acc
        });

    let gamma = &most_common_bits
        .into_iter()
        .map(|bit| bit >= 0)
        .collect::<Vec<_>>();

    let epsilon = &gamma.into_iter().map(|bit| !bit).collect::<Vec<_>>();

    binary_arr_to_u32(gamma) * binary_arr_to_u32(epsilon)
}

fn main() {
    let binary_numbers = parse_file("input/03.txt");
    println!("Solution A: {}", solution_a(&binary_numbers));
    println!("Solution B: {}", solution_b(&binary_numbers));
}
