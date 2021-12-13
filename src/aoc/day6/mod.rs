use crate::Problem;
use std::collections::VecDeque;

pub struct Day6 {}

impl Problem for Day6 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day6/input.txt".to_owned());
        let input: Vec<i32> = data
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        Self::calculate(&input, 80)
    }

    fn part_two(&self) -> String {
        let data = self.input_as_string("src/aoc/day6/input.txt".to_owned());
        let input: Vec<i32> = data
            .split(",")
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect();

        Self::calculate(&input, 256)
    }
}

impl Day6 {
    fn run_timer(q: &mut VecDeque<u128>) {
        let temp = q.pop_front().unwrap();
        q.push_back(temp);
        q[6] += temp;
    }

    fn init_queue() -> VecDeque<u128> {
        let mut q = VecDeque::new();
        for i in 0..9 {
            q.push_back(0)
        }
        q
    }

    fn calculate(input: &Vec<i32>, count: i32) -> String {
        let mut q: VecDeque<u128> = Self::init_queue();
        for i in input {
            q[*i as usize] += 1;
        }

        for i in 0..count {
            Self::run_timer(&mut q);
        }
        q.iter().sum::<u128>().to_string()
    }
}
