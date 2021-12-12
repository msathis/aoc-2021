use crate::Problem;

pub struct Day6 {}

impl Problem for Day6 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day6/input.txt".to_owned());
        let mut input: Vec<i32> = data
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        for i in 0..80 {
            Self::run_timer(&mut input);
        }
        input.len().to_string()
    }

    fn part_two(&self) -> String {
        todo!()
    }
}

impl Day6 {
    fn run_timer(input: &mut Vec<i32>) {
        let mut count = 0;
        for i in 0..input.len() {
            if input[i] == 0 {
                count += 1;
                input[i] = 6;
            } else {
                input[i] -= 1;
            }
        }
        for i in 0..count {
            input.push(8);
        }
    }
}
