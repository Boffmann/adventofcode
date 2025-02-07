extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::{
  IResult,
  character::complete::{digit1, space1},
  sequence::separated_pair,
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}


fn main() {
    let input = read_input("./input_part1.txt");
    let mut result = 0;
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    for line in &input {
        let numbers = parse_elements(line).expect("Some valid numbers").1;
        list1.push(numbers.0);
        list2.push(numbers.1);
    }
    list1.sort();
    list2.sort();
    for element in &list1 {
        
        result = result + element * count_ocurrences_in(&list2, element);
    }
    println!("{:?}", result);
}

fn count_ocurrences_in(list: &Vec<i32>, number: &i32) -> i32 {
    list.iter().filter(|&element| *element == *number).count() as i32
}

fn parse_elements(element_string: &str) -> IResult<&str, (i32, i32)> {
    let (remainder, numbers) = separated_pair(digit1, space1, digit1)(element_string)
        .map(|(remainder, parsed)| {
            (remainder, (parsed.0.parse::<i32>().unwrap(), parsed.1.parse::<i32>().unwrap()))
        })?;
    Ok((remainder, (numbers.0, numbers.1)))
}