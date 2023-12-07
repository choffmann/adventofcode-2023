use regex::Regex;
use adventofcode_2023::days::Day;
use adventofcode_2023::utils::{Folder, read_file};

const CURRENT_DAY: Day = Day(2);

#[derive(Debug)]
struct CubeRound {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    round: u32,
    cubes: Vec<CubeRound>,
}

impl Game {
    pub fn new(line: &str) -> Self {
        let regex = Regex::new(r"Game (?P<round>\d+): (?P<cubes>.*$)").unwrap();
        let Some(caps) = regex.captures(line) else { return panic!("Error while parsing string"); };
        let round = &caps["round"];
        let round = round.parse::<u32>().expect("Error while parsing game round");
        let cubes = &caps["cubes"];
        let cubes = cubes.split("; ")
            .map(|round| {
                let mut red: u32 = 0;
                let mut green: u32 = 0;
                let mut blue: u32 = 0;
                let regex = Regex::new(r"(?P<n>\d+) (?P<color>red|green|blue)").unwrap();
                for (_, [n, color]) in regex.captures_iter(round).map(|c| c.extract()) {
                    match color {
                        "red" => red = n.parse().expect("Error while parsing value of round"),
                        "blue" => blue = n.parse().expect("Error while parsing value of round"),
                        "green" => green = n.parse().expect("Error while parsing value of round"),
                        _ => panic!("No matching color red|green|blue")
                    }
                };
                CubeRound { red, green, blue }
            })
            .collect::<Vec<CubeRound>>();

        Game { round, cubes }
    }
}

fn main() {
    println!("Running Day {CURRENT_DAY}");
    let result_part_one = part_one(read_file(Folder::Input, CURRENT_DAY).as_str());
    let result_part_two = part_two(read_file(Folder::Input, CURRENT_DAY).as_str());
    println!("  Part 1: {:?}", result_part_one);
    println!("  Part 2: {:?}", result_part_two);
}

fn part_one(input: &str) -> Option<u32> {
    let contained_cubes = CubeRound { red: 12, green: 13, blue: 14 };
    let result: u32 = input.lines()
        .map(|line| Game::new(line))
        .filter(|game| {
            let filtered_list: Vec<&CubeRound> = game.cubes.iter().filter(|cube| cube.blue <= contained_cubes.blue && cube.green <= contained_cubes.green && cube.red <= contained_cubes.red).collect();
            filtered_list.len() == game.cubes.len()
        })
        .map(|game| game.round)
        .sum();
    Some(result)
}

fn part_two(input: &str) -> Option<u32> {
    let result: u32 = input.lines()
        .map(|line| Game::new(line))
        .map(|game| {
            let mut result: CubeRound = CubeRound { red: 0, green: 0, blue: 0 };
            game.cubes.into_iter().for_each(|cube| {
                if cube.green > result.green { result.green = cube.green }
                if cube.red > result.red { result.red = cube.red }
                if cube.blue > result.blue { result.blue = cube.blue }
            });
            return result;
        })
        .map(|cube| cube.blue * cube.red * cube.green)
        .sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use adventofcode_2023::utils::Folder;
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(read_file(Folder::Examples, CURRENT_DAY).as_str());
        assert_eq!(result, Some(8))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(read_file(Folder::Examples, CURRENT_DAY).as_str());
        assert_eq!(result, Some(2286))
    }
}
