use adventofcode_2023::days::Day;
use adventofcode_2023::utils::{Folder, read_file, read_file_part};

const CURRENT_DAY: Day = Day(1);

fn main() {
    println!("Running Day 01");

    let result_part_one = part_one(read_file(Folder::Input, CURRENT_DAY).as_str());
    let result_part_two = part_two(read_file(Folder::Input, CURRENT_DAY).as_str());

    println!("  Part 1: {:?}", result_part_one);
    println!("  Part 2: {:?}", result_part_two);
}


fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input.lines()
        .map(|line| line.chars().filter(|char| char.is_numeric()).collect::<String>())
        .map(|n| concat_numbers(n).parse::<u32>().expect("Error while parsing into number"))
        .sum();

    return Some(result);
}

fn part_two(input: &str) -> Option<u32> {
    let result: u32 = input.lines()
        .map(|str| {
            str.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .map(|line| line.chars().filter(|char| char.is_numeric()).collect::<String>())
        .map(|n| concat_numbers(n).parse::<u32>().expect("Error while parsing string to number"))
        .sum();
    Some(result)
}

fn concat_numbers(str: String) -> String {
    return if str.len() == 1 {
        format!("{}{}", str, str)
    } else {
        let first = str.chars().nth(0);
        let last = str.chars().last();
        format!("{}{}", first.unwrap(), last.unwrap())
    };
}

#[cfg(test)]
mod tests {
    use adventofcode_2023::utils::Folder;
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(read_file_part(Folder::Examples, CURRENT_DAY, 1).as_str());
        assert_eq!(result, Some(142))
    }

    #[test]
    fn test_part_two() {
        let result = part_two(read_file_part(Folder::Examples, CURRENT_DAY, 2).as_str());
        assert_eq!(result, Some(281))
    }
}