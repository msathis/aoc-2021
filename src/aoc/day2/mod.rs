use crate::aoc::problem::Problem;

pub struct Day2 {}

impl Problem for Day2 {
    fn part_one(&self) -> String {
        let data = self.input_as_vec("src/aoc/day2/input.txt".to_owned());

        let res = data
            .iter()
            .map(|s| {
                let parts: Vec<&str> = s.split(' ').collect();
                let val = parts[1].parse::<i32>().unwrap();
                match parts[0] {
                    "forward" => return (val, 0),
                    "up" => return (0, -val),
                    "down" => return (0, val),
                    &_ => {
                        panic!("Wrong direction");
                    }
                }
            })
            .fold((0, 0), |acc, curr| {
                return (acc.0 + curr.0, acc.1 + curr.1);
            });
        return (res.0 * res.1).to_string();
    }

    fn part_two(&self) -> String {
        todo!()
    }
}
