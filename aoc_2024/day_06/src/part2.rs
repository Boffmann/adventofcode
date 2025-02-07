extern crate nom;
use std::char;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::fs::File;
use nom::character;
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
    let input = read_input("./input_test.txt");
    let mut parsed_map: Vec<Vec<MapElement>> = Vec::new();
    let mut result = 0;
    for line in &input {
        let parsed_element: Vec<MapElement> = line.chars()
        .map(|c| MapElement {
            character: c,
            is_known_loop: HashSet::new()
        })
        .collect();
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
    let mut check_loop_direction = rotate_direction(&direction);
    loop {
        let next_position = update_coordinates(x, y, &direction);
        let next_potential_loop_position = update_coordinates(x, y, &check_loop_direction);
        match map.get_position(next_potential_loop_position.0, next_potential_loop_position.1) {
            Some(mapElement) => {
                if mapElement.character != '#' {
                    if is_loop(&mut map, x, y, &check_loop_direction) {
                        result = result + 1;
                        println!("{:?}", next_position);
                    }
                }
            },
            None => {}
        }

        match map.get_position(next_position.0, next_position.1) {
            Some(mapElement) => {
                if mapElement.character == '#' {
                    direction = rotate_direction(&direction);
                    check_loop_direction = rotate_direction(&direction);
                } else {
                    x = next_position.0;
                    y = next_position.1;
                }
            },
            None => break
        }
    }
    map.print_map();
    println!("{:?}", result);
}

fn update_coordinates(x: i32, y: i32, direction: &Direction) -> (i32, i32) {
    return match direction {
        Direction::UP => (x, y - 1),
        Direction::RIGHT => (x + 1, y),
        Direction::DOWN => (x, y + 1),
        Direction::LEFT => (x - 1, y),
    };
}

fn is_loop(map: &mut Map, start_x: i32, start_y: i32, start_direction: &Direction) -> bool {
    let mut x = start_x;
    let mut y = start_y;
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut direction: Direction = start_direction.clone();
    loop {
        let check_at = update_coordinates(x, y, &direction);
        if visited.contains(&check_at) && (check_at.0 != start_x || check_at.1 != start_y) {
            return false;
        }
        visited.push((x, y));
        match map.get_position(check_at.0, check_at.1) {
            Some(mapElement) => {
                if mapElement.is_known_loop.contains(&direction) {
                    println!("Shortcut");
                    return true;
                }
                if mapElement.character == '#' {
                    direction = rotate_direction(&direction);
                } else {
                    x = check_at.0;
                    y = check_at.1;
                }
            },
            None => return false
        }
        if x == start_x && y == start_y {
            mark_known_loop(map, start_x, start_y, start_direction);
            return true;
        }
    }
}

fn mark_known_loop(map: &mut Map, start_x: i32, start_y: i32, direction: &Direction) {
    let mut x = start_x;
    let mut y = start_y;
    let mut direction: Direction = direction.clone();
    loop {
        map.update_position_known_loop(x, y, &direction);
        let check_at = update_coordinates(x, y, &direction);
        match map.get_position(check_at.0, check_at.1) {
            Some(mapElement) => {
                if mapElement.character == '#' {
                    direction = rotate_direction(&direction);
                } else {
                    x = check_at.0;
                    y = check_at.1;
                }
            },
            None => println!("This should be a loop and as such not reachable")
        }
        if x == start_x && y == start_y {
            return;
        }
    }
}

fn rotate_direction(direction: &Direction) -> Direction {
    return match direction {
        
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Debug)]
struct MapElement {
    character: char,
    is_known_loop: HashSet<Direction>,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<MapElement>>,
    visited: i32,
}

impl Map {
    fn get_position(&self, x: i32, y: i32) -> Option<&MapElement> {
        if !self.in_bound(x, y) {
            return None;
        }
        Some(&self.map[y as usize][x as usize])
    }

    fn in_bound(&self, x: i32, y: i32) -> bool { 
        x < self.map[0].len() as i32 &&
        y < self.map.len() as i32 &&
        x >= 0 &&
        y >= 0
    }

    fn find_start(&self) -> (i32, i32) {
        for y in 0..self.map.len() {
            let row = &self.map[y];
            for x in 0..row.len() {
                if (row[x].character == '^') {
                    return (x as i32, y as i32) 
                }
            }
        }
        (0, 0)
    }

    fn update_position_known_loop(&mut self, x: i32, y: i32, direction: &Direction) {
        self.map[y as usize][x as usize].is_known_loop.insert(direction.clone());
    }

    fn print_map(&self) {
        for line in &self.map {
            let s: String = line.into_iter().map(|mapElement| mapElement.character).collect();
            println!("{:?}", s);
        }
    }
}