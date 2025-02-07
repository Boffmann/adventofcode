extern crate nom;
use std::char;
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
    let mut parsed_map: Vec<Vec<char>> = Vec::new();
    for line in &input {
        let parsed_element = line.chars().collect();
        parsed_map.push(parsed_element);
    }
    let mut map = Map {
        map: parsed_map,
        visited: 0,
    };
    let start = map.find_start();
    let mut x = start.0;
    let mut y = start.1;
    let mut direction = Direction::UP;
    loop {
        map.update_position(x, y);
        let check_at = match direction {
            Direction::UP => (x, y - 1),
            Direction::RIGHT => (x + 1, y),
            Direction::DOWN => (x, y + 1),
            Direction::LEFT => (x - 1, y),
        };
        match map.get_position(check_at.0, check_at.1) {
            Some(character) => {
                if character == '#' {
                    direction = rotate_direction(&direction);
                } else {
                    x = check_at.0;
                    y = check_at.1;
                }
            },
            None => break
        }
    }
    map.print_map();
    println!("{:?}", map.visited);
}

fn rotate_direction(direction: &Direction) -> Direction {
    return match direction {
        
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

struct Map {
    map: Vec<Vec<char>>,
    visited: i32,
}

impl Map {
    fn get_position(&self, x: usize, y: usize) -> Option<char> {
        if !self.in_bound(x, y) {
            return None;
        }
        Some(self.map[y][x])
    }

    fn in_bound(&self, x: usize, y: usize) -> bool {
        x < self.map[0].len() && y < self.map.len()
    }

    fn find_start(&self) -> (usize, usize) {
        for y in 0..self.map.len() {
            let row = &self.map[y];
            for x in 0..row.len() {
                if (row[x] == '^') {
                    return (x, y)
                }
            }
        }
        (0, 0)
    }

    fn update_position(&mut self, x: usize, y: usize) {
        match self.get_position(x, y) {
            Some(character) => {
                if character!= 'X' {
                    self.map[y][x] = 'X';
                    self.visited = self.visited + 1;
                }
            }
            None => println!("Error out of bounds")
        }
    }

    fn print_map(&self) {
        for line in &self.map {
            let s: String = line.into_iter().collect();
            println!("{:?}", s);
        }
    }
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