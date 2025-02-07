extern crate nom;
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
    let input = read_input("input_part1.txt");
    let mut pipes: HashMap<Coordinate, Pipe> = HashMap::new();
    let mut start_coordinate: Coordinate = Coordinate{x: 0, y: 0};

    for line_index in 0..input.len() {
        let line = &input[line_index];
        for char_index in 0..line.len() {
            let character = line.chars().nth(char_index).unwrap();
            let pipe = parse_pipe(character).unwrap();
            let coordinate = Coordinate{x: char_index, y: line_index};
            if pipe == Pipe::Start {
                start_coordinate = coordinate.clone();
            }
            pipes.insert(coordinate, pipe);
        }
        // let extrapolated = process_history(parse_history(line).expect("A history").1, 0);
        // result = result + extrapolated;
    }
    let starts = find_starts(&pipes, start_coordinate);
    for start in starts {
        let mut current_distance = 0;
        let mut current_coordinate = start.0;
        let mut current_direction = start.1;
        loop {
            current_distance = current_distance + 1;
            let present_distance = results.get(&current_coordinate).unwrap_or(&std::u32::MAX);
            results.insert(current_coordinate.clone(), std::cmp::min(*present_distance, current_distance));
            let traveled = travel(&pipes, &current_coordinate, &current_direction);
            current_coordinate = traveled.0;
            current_direction = traveled.1;
            if *pipes.get(&current_coordinate).unwrap() == Pipe::Start {
                break;
            }
        }
    }
    let all_scores = results.values().cloned().collect::<Vec<u32>>();
    let result = *all_scores.iter().max().unwrap();
    assert!(result == 6820, "Result mismatch");
    println!("{:?}", result);

    // println!("{:?}", pipes);
    // println!("{:?}", travel(&pipes, &Coordinate{x: 1, y: 1}, &Direction::North));
}

fn find_starts(pipes: &HashMap<Coordinate, Pipe>, start_coordinate: Coordinate) -> Vec<(Coordinate, Direction)> {
    let mut result: Vec<(Coordinate, Direction)> = Vec::new();
    let north_coordinate = Coordinate{x: start_coordinate.x, y: start_coordinate.y - 1};
    let east_coordinate = Coordinate{x: start_coordinate.x + 1, y: start_coordinate.y};
    let south_coordinate = Coordinate{x: start_coordinate.x, y: start_coordinate.y + 1};
    let west_coordinate = Coordinate{x: start_coordinate.x - 1, y: start_coordinate.y};

    if *pipes.get(&north_coordinate).unwrap() == Pipe::Vertical
         || *pipes.get(&north_coordinate).unwrap() == Pipe::SouthWest
         || *pipes.get(&north_coordinate).unwrap() == Pipe::SouthEast {
        result.push((north_coordinate, Direction::North));
    }

    if *pipes.get(&east_coordinate).unwrap() == Pipe::Horizontal
         || *pipes.get(&east_coordinate).unwrap() == Pipe::NorthWest
         || *pipes.get(&east_coordinate).unwrap() == Pipe::SouthWest {
        result.push((east_coordinate, Direction::East));
    }

    if *pipes.get(&south_coordinate).unwrap() == Pipe::Vertical
         || *pipes.get(&south_coordinate).unwrap() == Pipe::NorthEast
         || *pipes.get(&south_coordinate).unwrap() == Pipe::NorthWest {
        result.push((south_coordinate, Direction::South));
    }

    if *pipes.get(&west_coordinate).unwrap() == Pipe::Horizontal
         || *pipes.get(&west_coordinate).unwrap() == Pipe::NorthEast
         || *pipes.get(&west_coordinate).unwrap() == Pipe::SouthEast {
        result.push((west_coordinate, Direction::West));
    }

    result
}

fn parse_pipe(unicode: char) -> Option<Pipe> {
    match unicode {
        '|' => Some(Pipe::Vertical),
        '-' => Some(Pipe::Horizontal),
        'L' => Some(Pipe::NorthEast),
        'J' => Some(Pipe::NorthWest),
        '7' => Some(Pipe::SouthWest),
        'F' => Some(Pipe::SouthEast),
        '.' => Some(Pipe::Ground),
        'S' => Some(Pipe::Start),
        _ => None
    }
}


#[derive(Eq, Hash, PartialEq, Debug)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn travel_south(pipe: &Pipe) -> Result<Direction, &'static str> {
    match pipe {
        Pipe::Vertical => Ok(Direction::South),
        Pipe::NorthEast => Ok(Direction::East),
        Pipe::NorthWest => Ok(Direction::West),
        _ => Err("Invalid pipe travel south")
    }
}

fn travel_west(pipe: &Pipe) -> Result<Direction, &'static str> {
    match pipe {
        Pipe::Horizontal => Ok(Direction::West),
        Pipe::NorthEast => Ok(Direction::North),
        Pipe::SouthEast => Ok(Direction::South),
        _ => Err("Invalid pipe travel west")
    }
}

fn travel_north(pipe: &Pipe) -> Result<Direction, &'static str> {
    match pipe {
        Pipe::Vertical => Ok(Direction::North),
        Pipe::SouthWest => Ok(Direction::West),
        Pipe::SouthEast => Ok(Direction::East),
        _ => Err("Invalid Pipe travel north")
    }
}

fn travel_east(pipe: &Pipe) -> Result<Direction, &'static str> {
    match pipe {
        Pipe::Horizontal => Ok(Direction::East),
        Pipe::NorthWest => Ok(Direction::North),
        Pipe::SouthWest => Ok(Direction::South),
        _ => Err("Invalid Pipe travel east")
    }
}

fn travel(pipes: &HashMap<Coordinate, Pipe>, current_location: &Coordinate, coming_from: &Direction) -> (Coordinate, Direction) {
    let current_pipe = pipes.get(current_location).expect("A pipe at location");
    let travel_direction = match coming_from {
        Direction::North => travel_north(current_pipe).expect("A direction"),
        Direction::East => travel_east(current_pipe).expect("A direction"),
        Direction::South => travel_south(current_pipe).expect("A direction"),
        Direction::West => travel_west(current_pipe).expect("A direction"),
    };

    let coordinate = match travel_direction {
        Direction::North => Coordinate{x: current_location.x, y: current_location.y - 1},
        Direction::East => Coordinate{x: current_location.x + 1, y: current_location.y},
        Direction::South => Coordinate{x: current_location.x, y: current_location.y + 1},
        Direction::West => Coordinate{x: current_location.x - 1, y: current_location.y},
    };

    return (coordinate, travel_direction);
}
