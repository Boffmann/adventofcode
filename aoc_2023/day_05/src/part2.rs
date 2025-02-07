extern crate nom;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::fs::File;
use nom::{
  IResult,
  character::complete::{digit1, space1},
  sequence::separated_pair,
  multi::separated_list1,
  bytes::complete::tag,
};

#[derive(Default, Debug, PartialEq, Eq)]
struct Range {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

#[derive(Default, Debug, PartialEq, Eq)]
struct Translator {
    ranges: Vec<Range>,
}

impl Translator {
    pub fn new() -> Self {
        Translator {
            ranges: Vec::new()
        }
    }

    fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    fn translate(&self, number: &u64) -> u64 {
        for range in &self.ranges {
            if range.source_start <= *number && range.source_start + range.length > *number {
                return range.dest_start + (*number - range.source_start);
            }
        }
        *number
    }
}

fn read_input(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    let reader = BufReader::new(File::open(filename).expect("cannot open input file"));

    for line in reader.lines() {
        result.push(line.unwrap())
    }
    return result
}

fn main() {
    let input = read_input("./input.txt");
    let seeds: Vec<u64> = parse_seeds(&input[0]).expect("Valid parsed seeds").1;
    let translators: HashMap<String, Translator> = parse_translators(&input);

    let mut result = std::u64::MAX;
    for i in (0..seeds.len()).step_by(2) {

        let mapped : u64 = match process_seed_range_binary_search(seeds[i], seeds[i] + seeds[i+1], &translators) {
            Ok(res) => res,
            Err(s) => panic!("Error: {}", s)
        };
        result = std::cmp::min(mapped, result);
    }
    
    println!("Lowest location number is: {}", result);
}

fn process_seed_range_binary_search(range_start: u64, range_end: u64, translators: &HashMap<String, Translator>) -> Result<u64, String> {
    if range_start > range_end {
        return Err("Start Range cannot be larger than end range".to_string());
    }
    let range_diff = range_end - range_start;
    if range_diff == 0 {
        return Ok(apply_translators(&range_start, translators));
    }
    let mapped_start = apply_translators(&range_start, translators);
    let mapped_end = apply_translators(&range_end, translators);
    let mapped_diff = if mapped_end > mapped_start {mapped_end - mapped_start} else {0};
    if mapped_diff == range_diff {
        return Ok(mapped_start);
    }
    let middle = (range_start + range_end) / 2;

    std::cmp::min(
        process_seed_range_binary_search(range_start, middle, translators),
        process_seed_range_binary_search(middle+1, range_end, translators)
    )
}

fn apply_translators(seed: &u64, translators: &HashMap<String, Translator>) -> u64 {
    let translator_names = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"];
    let mut result = *seed;
    for translator_name in translator_names {
        result = match translators.get(translator_name) {
            Some(translator) => translator.translate(&result),
            None => {
                println!("Error No translator found for name {}", translator_name);
                0
            }
        }
    }
    result
}

fn parse_translators(input_lines: &Vec<String>) -> HashMap<String, Translator> {
    let mut translators: HashMap<String, Translator> = HashMap::new();
    let mut current_translator: &mut Translator = &mut Translator::new();
    for line_index in 1..input_lines.len() {
        let line = &input_lines[line_index as usize];
        if line == "" {
            continue;
        }
        if line.contains("map:") {
            let translator_name = line.replace(" map:", "").to_string();
            translators.insert(translator_name.clone(), Translator::new());
            current_translator = translators.get_mut(&translator_name).unwrap();
            continue;
        }
        current_translator.add_range(parse_range(&line).expect("A valid range").1);
    }
    translators
}

fn parse_seeds(seed_string: &str) -> IResult<&str, Vec<u64>> {
    assert!(seed_string.contains("seeds"));
    let (remainder, seeds) = separated_pair(tag("seeds"), tag(": "),
        separated_list1(space1, digit1))(seed_string)
        .map(|(remainder, seeds)| {
            (remainder, seeds.1.iter()
             .map(|seed| seed.parse::<u64>().expect("An u64"))
             .collect::<Vec<u64>>())
        })?;
    Ok((remainder, seeds))
}

fn parse_range(range_string: &str) -> IResult<&str, Range> {
    let (remainder, ranges) = separated_list1(space1, digit1)(range_string)?;
    Ok((remainder, Range{
        source_start: ranges[1].parse::<u64>().expect("A number"),
        dest_start: ranges[0].parse::<u64>().expect("A number"),
        length: ranges[2].parse::<u64>().expect("A number"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_translation() {
        let mut test_translation = Translator::new();
        assert_eq!(test_translation.translate(&2), 2);
        assert_eq!(test_translation.translate(&3), 3);
        assert_eq!(test_translation.translate(&5), 5);
        assert_eq!(test_translation.translate(&6), 6);
        test_translation.add_range(Range{source_start: 2, dest_start: 5, length: 2});
        assert_eq!(test_translation.translate(&2), 5);
        assert_eq!(test_translation.translate(&3), 6);
        assert_eq!(test_translation.translate(&5), 5);
        assert_eq!(test_translation.translate(&6), 6);
        test_translation.add_range(Range{source_start: 5, dest_start: 10, length: 2});
        assert_eq!(test_translation.translate(&2), 5);
        assert_eq!(test_translation.translate(&3), 6);
        assert_eq!(test_translation.translate(&5), 10);
        assert_eq!(test_translation.translate(&6), 11);
    }

    #[test]
    fn test_parse_range() {
        let range_string = "45 77 23";
        let mut test_translation = Translator::new();
        assert_eq!(test_translation.translate(&77), 77);
        assert_eq!(test_translation.translate(&82), 82);
        assert_eq!(test_translation.translate(&87), 87);
        assert_eq!(test_translation.translate(&92), 92);
        let range = parse_range(range_string).expect("A range").1;
        test_translation.add_range(range);
        assert_eq!(test_translation.translate(&77), 45);
        assert_eq!(test_translation.translate(&82), 50);
        assert_eq!(test_translation.translate(&87), 55);
        assert_eq!(test_translation.translate(&92), 60);
    }

    #[test]
    fn test_parse_seeds() {
        let seeds_string = "seeds: 79 14 55 13";
        let seeds = parse_seeds(seeds_string).expect("Some valid Seeds").1;
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_seed_to_location() {
        println!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt"));
        
        let input = read_input(concat!(env!("CARGO_MANIFEST_DIR"), "/src/input.txt"));
        let translators: HashMap<String, Translator> = parse_translators(&input);
        assert_eq!(apply_translators(&3281178213, &translators), 69323688);
    }
}
