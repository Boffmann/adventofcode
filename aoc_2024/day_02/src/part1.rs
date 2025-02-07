extern crate nom;
use std::io::{BufRead, BufReader};
use std::fs::File;
use nom::multi::separated_list1;
use nom::{
  IResult,
  character::complete::{digit1, space1},
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
    for line in &input {
        let mut report: Vec<i32> = parse_report(line).expect("A report").1;
        if is_save(&mut report) {
            result = result + 1;
        }
    }
    println!("{:?}", result);
}

fn is_save(report: &mut Vec<i32>) -> bool {
    if report[0] < report[report.len() - 1] {
        return is_save_increasing(report);
    } else if report[0] > report[report.len() - 1] {
        report.reverse();
        return is_save_increasing(report);
    } else {
        return false;
    }
}

fn is_save_increasing(report: &Vec<i32>) -> bool {
    for report_index in 0..report.len() - 1 {
        let report_diff = report[report_index + 1] - report[report_index];
        if report_diff <= 0 || report_diff > 3 {
            return false;
        }
    }
    true
}

fn parse_report(report_string: &str) -> IResult<&str, Vec<i32>> {
    let (remainder, report) = separated_list1(space1, digit1)(report_string)
        .map(|(remainder, report)| {
            (remainder, report.iter().map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        })?;
    Ok((remainder, report))
}