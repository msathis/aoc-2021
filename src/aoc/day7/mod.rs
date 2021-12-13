use crate::Problem;

pub struct Day7 {}

impl Problem for Day7 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day7/input.txt".to_owned());
        let mut input: Vec<i32> = data
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();
        input.sort();

        let median = if input.len() % 2 == 0 {
            let index = input.len() / 2;
            (input[index] + input[index - 1]) / 2
        } else {
            let index = (input.len() / 2) + 1;
            input[index]
        };

        input
            .iter()
            .map(|x| (x - median).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        todo!()
    }
}
