use crate::aoc::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn part_one(&self) -> String {
        let data: Vec<i32> = self
            .input_as_vec("src/aoc/day1/input.txt".to_owned())
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let res = data
            .iter()
            .zip(data.iter().skip(1))
            .map(|(a, b)| {
                if a < b {
                    return 1;
                }
                0
            })
            .fold(0, |acc, x| acc + x);
        res.to_string()
    }

    fn part_two(&self) -> String {
        "Welcome to day1 - part 2".to_owned()
    }
}
