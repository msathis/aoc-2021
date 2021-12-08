use crate::aoc::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn part_one(&self) -> String {
        let data = self.get_int_vec();
        let res = data
            .iter()
            .zip(data.iter().skip(1))
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .fold(0, |acc, x| acc + x);
        res.to_string()
    }

    fn part_two(&self) -> String {
        let data = self.get_int_vec();
        let sums: Vec<i32> = data
            .iter()
            .zip(data.iter().skip(1))
            .zip(data.iter().skip(2))
            .map(|((a, b), c)| a + b + c)
            .collect();

        let res = sums
            .iter()
            .zip(sums.iter().skip(1))
            .map(|(a, b)| if a < b { 1 } else { 0 })
            .fold(0, |acc, x| acc + x);
        res.to_string()
    }
}

impl Day1 {
    fn get_int_vec(&self) -> Vec<i32> {
        let data: Vec<i32> = self
            .input_as_vec("src/aoc/day1/input.txt".to_owned())
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        data
    }
}
