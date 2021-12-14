use crate::Problem;
use std::collections::HashMap;

pub struct Day10 {}

impl Problem for Day10 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day10/input.txt".to_owned());
        let lines = data
            .lines()
            .map(|x| x.split("").filter(|p| !p.is_empty()).collect::<Vec<&str>>())
            .collect::<Vec<Vec<_>>>();
        let map = HashMap::from([
            (")".to_owned(), 3),
            ("]".to_owned(), 57),
            ("}".to_owned(), 1197),
            (">".to_owned(), 25137),
        ]);

        let mut sum = 0;
        for line in lines {
            let mut stack = vec![];
            for ch in line {
                if map.contains_key(ch) {
                    let last = stack.pop();
                    if last.is_none() {
                        break;
                    }
                    let last = last.unwrap();
                    if (last == "{" && ch == "}")
                        || (last == "(" && ch == ")")
                        || (last == "[" && ch == "]")
                        || (last == "<" && ch == ">")
                    {
                        continue;
                    } else {
                        sum += map.get(ch).unwrap();
                    }
                } else {
                    stack.push(ch);
                }
            }
        }

        return sum.to_string();
    }

    fn part_two(&self) -> String {
        todo!()
    }
}