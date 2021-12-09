use crate::Problem;
use std::ops::BitXor;

pub struct Day3 {}

impl Problem for Day3 {
    fn part_one(&self) -> String {
        let file = self.input_as_string("src/aoc/day3/input.txt".to_owned());
        let lines = file.lines();
        let mut array = [0; 12];
        let size = lines
            .inspect(|l| {
                let parts: Vec<&str> = l.split("").skip(1).collect();
                for (i, v) in parts.iter().enumerate() {
                    let val = match *v {
                        "0" => 0,
                        "1" => 1,
                        _ => 0,
                    };
                    if val > 0 {
                        array[i] = array[i] + val;
                    }
                }
            })
            .count();
        let gamma_idx = array
            .iter()
            .map(|v| if *v > (size / 2) as i32 { "1" } else { "0" })
            .collect::<String>();
        let gamma = isize::from_str_radix(gamma_idx.as_str(), 2).unwrap();
        let epsilon = gamma.bitxor(4095);

        return (gamma * epsilon).to_string();
    }

    fn part_two(&self) -> String {
        todo!()
    }
}
