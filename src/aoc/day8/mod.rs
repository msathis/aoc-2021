use crate::Problem;
use std::collections::HashMap;
use std::str::FromStr;

pub struct Day8 {}

struct TestCase {
    sample: Vec<String>,
    input: Vec<String>,
}

impl FromStr for TestCase {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" | ").collect::<Vec<&str>>();
        let sample = parts[0]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let input = parts[1]
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        Ok(Self { sample, input })
    }
}

impl Problem for Day8 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day8/input.txt".to_owned());
        let count = data
            .lines()
            .map(|x| x.split(" ").skip(11).collect::<Vec<&str>>())
            .flatten()
            .filter(|x| return x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
        return count.to_string();
    }

    fn part_two(&self) -> String {
        let data = self.input_as_string("src/aoc/day8/input.txt".to_owned());
        let tcs = data
            .lines()
            .map(|x| x.parse::<TestCase>())
            .collect::<Vec<_>>();
        let map = HashMap::from([
            (0, "abcefg"),
            (1, "cf"),
            (2, "acdeg"),
            (3, "acdfg"),
            (4, "bcdf"),
            (5, "abdfg"),
            (6, "abdefg"),
            (7, "acf"),
            (8, "abcdefg"),
            (9, "abcdfg"),
        ]);

        let mut result = 0;
        for tc in &tcs {}

        result.to_string()
    }
}
