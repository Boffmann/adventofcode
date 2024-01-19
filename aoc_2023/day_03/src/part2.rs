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

#[derive(Default, Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Number {
    value: u32,
    x_bounds: Point,
    y_bounds: Point
}

fn main() {
    let input = read_input("./input.txt");
    let mut gears: Vec<Point> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();
    let mut x;
    let mut y = 0;
    let mut num_start_x = 0;
    for line in input {
        x = -1;
        let mut number = "".to_string();
        for c in line.chars() {
            x += 1;
            if c.is_numeric() {
                if number == "" {
                    num_start_x = x;
                }
                number.push(c);
            } else {
                push_back_number(&mut numbers, &mut number, num_start_x, x, y);
                if c != '*' {
                    continue;
                }
                gears.push(Point{x: x, y: y});
                // println!("{:?}", c);
            }
        }
        push_back_number(&mut numbers, &mut number, num_start_x, x, y);
        y += 1;
    }

    let mut result: u32 = 0;
    for gear in &gears {
        result += get_gear_number(gear, &numbers);
    }
    println!("{:?}", result);
}

fn push_back_number(numbers: &mut Vec<Number>, number: &mut String, num_start_x: i32, x: i32, y: i32) {
    if number != "" {
        let push_number =
            Number{
                value: number.parse::<u32>().unwrap(),
                x_bounds: Point{x: num_start_x - 1, y: x},
                y_bounds: Point{x: y - 1, y: y + 1}};
        // println!("{:?}", push_number);
        numbers.push(push_number);
        *number = "".to_string();
    }
}

fn get_gear_number(gear: &Point, numbers: &Vec<Number>) -> u32 {
    let mut result = 1;
    let mut num_gears = 0;
    for number in numbers {
        if number.x_bounds.x <= gear.x && number.x_bounds.y >= gear.x &&
            number.y_bounds.x <= gear.y && number.y_bounds.y >= gear.y {
            result *= number.value;
            num_gears += 1;
        }
    }
    if num_gears == 2 {
        return result
    }
    0
}

