// extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}


fn main() {
    let input = read_input("./input_2.txt");
    let mut result : u32 = 0;
    for line in input {
        result += extract_number(&line);
    }
    println!("{result}");
}

fn translate_number_string(number_string: &str) -> String {
    let number_translations: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    match number_translations.get(number_string) {
        Some(number) => return number.to_string(),
        None => return number_string.to_string()
    }
}

fn extract_number(input: &str) -> u32 {
    let all_numbers = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut number_with_position = all_numbers.iter()
        .flat_map(|number| input.match_indices(number))
        .collect::<Vec<(usize, &str)>>();

    number_with_position.sort_by(|(position, _number), (position2, _number2)| position.cmp(&position2));
    let result_numbers = number_with_position.iter()
        .map(|(_position, number)| number)
        .map(|number| translate_number_string(number))
        .collect::<Vec<String>>();
    format!("{}{}", result_numbers[0], result_numbers[result_numbers.len() - 1])
        .parse::<u32>().unwrap()
}

