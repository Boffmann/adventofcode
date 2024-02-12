// extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}


fn main() {
    let input = read_input("./input_1.txt");
    let mut result : u32 = 0;
    for line in input {
        result += extract_number(&line);
    }
    println!("{result}");
}

fn extract_number(input: &str) -> u32 {
    let all_numbers = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut number_with_position = all_numbers.iter()
        .flat_map(|number| input.match_indices(number))
        .collect::<Vec<(usize, &str)>>();

    number_with_position.sort_by(|(position, _number), (position2, _number2)| position.cmp(&position2));
    let result_numbers = number_with_position.iter()
        .map(|(_position, number)| number.to_string())
        .collect::<Vec<String>>();
    let mut result : String = "".to_owned();
    result.push_str(&result_numbers[0]);
    result.push_str(&result_numbers[result_numbers.len() - 1]);
    result.parse::<u32>().unwrap()
}

