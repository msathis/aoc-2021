use crate::Problem;
use std::collections::HashMap;

pub struct Day10 {}

impl Problem for Day10 {
    fn part_one(&self) -> String {
        let lines = self.parse_input();
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
        let lines = self.parse_input();
        let map = HashMap::from([
            ("(".to_owned(), 1),
            ("[".to_owned(), 2),
            ("{".to_owned(), 3),
            ("<".to_owned(), 4),
        ]);

        let mut res = vec![];

        for line in lines {
            let mut stack = vec![];
            let mut total: u128 = 0;
            let mut is_valid = true;

            for ch in line {
                if !map.contains_key(ch) {
                    let last = stack.last();
                    if last.is_none() {
                        break;
                    }
                    let last = last.unwrap();
                    if (last == &"{" && ch == "}")
                        || (last == &"(" && ch == ")")
                        || (last == &"[" && ch == "]")
                        || (last == &"<" && ch == ">")
                    {
                        stack.pop();
                    } else {
                        is_valid = false;
                        break;
                    }
                } else {
                    stack.push(ch);
                }
            }

            if is_valid {
                stack.reverse();
                for ch in stack {
                    total = (total * 5) as u128 + map.get(ch).unwrap();
                }
                res.push(total);
            }
        }

        res.sort();
        return res[res.len() / 2].to_string();
    }
}

impl Day10 {
    fn parse_input(&self) -> Vec<Vec<&str>> {
        let data = self.input_as_string("src/aoc/day10/input.txt".to_owned());
        let lines = data
            .lines()
            .map(|x| x.split("").filter(|p| !p.is_empty()).collect::<Vec<&str>>())
            .collect::<Vec<Vec<_>>>();
        lines
    }
}
