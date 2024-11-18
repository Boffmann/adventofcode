extern crate nom;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use nom::bytes::complete::tag;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::{
  IResult,
  character::complete::{alphanumeric1},
};

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

#[derive(Debug)]
struct Crossway {
    key: String,
    left: String,
    right: String,
}

impl Crossway {
    fn is_start(&self) -> bool {
        return self.key.chars().nth(2).unwrap() == 'A';
    }
    fn is_end(&self) -> bool {
        return self.key.chars().nth(2).unwrap() == 'Z';
    }
}

fn calculate_loop_size(crossway: &Crossway, crossways: &HashMap<String, Crossway>, directions: &str) -> usize {
    let mut direction_index = 0;
    let mut current_crossway = crossway;
    while !current_crossway.is_end() {
        if directions.chars().nth(direction_index % directions.len()).unwrap() == 'R' {
            current_crossway = crossways.get(&current_crossway.right).unwrap();
        } else {
            current_crossway = crossways.get(&current_crossway.left).unwrap();
        }
        direction_index = direction_index + 1;
    }
    direction_index
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let input = read_input("input_part2.txt");
    let mut crossways: HashMap<String, Crossway> = HashMap::new();
    let directions: &str = input.get(0).expect("Direction String");
    for line_index in 2..input.len() {
        let crossway = parse_crossway(&input[line_index]).expect("A Crossway").1;
        crossways.insert(crossway.key.clone(), crossway);
    }
    let mut loop_sizes: Vec<usize> = Vec::new();

    for (_key, crossway) in &crossways {
        if crossway.is_start() {
            loop_sizes.push(calculate_loop_size(crossway, &crossways, directions));
        }
    }
    println!("{:?}", lcm(&loop_sizes));
}

fn parse_crossway(crossway_string: &str) -> IResult<&str, Crossway> {
    let (remainder, crossway) = separated_pair(alphanumeric1, tag(" = "),
        delimited(tag("("), separated_pair(alphanumeric1, tag(", "), alphanumeric1), tag(")")))(crossway_string)
        .map(|parsed| {
            let crossway = Crossway {
                key: parsed.1.0.to_string(),
                left: parsed.1.1.0.to_string(),
                right: parsed.1.1.1.to_string(),
            };
            (parsed.0, crossway)
        })?;
    Ok((remainder, crossway))
}
