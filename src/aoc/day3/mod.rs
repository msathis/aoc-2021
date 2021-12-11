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
        let file = self.input_as_string("src/aoc/day3/input.txt".to_owned());
        let data = file.lines().map(|x| x).collect::<Vec<&str>>();

        let mut oxygen = 0;
        let mut co2 = 0;
        let mut oxy_str = "".to_owned();
        let mut co2_str = "".to_owned();
        let mut oxy_filtered = data.clone();
        let mut co2_filtered = data.clone();

        for i in 0..12 {
            let oxy_sum = oxy_filtered
                .iter()
                .map(|s| {
                    let char = s.as_bytes()[i] as char;
                    match char {
                        '0' => 0,
                        '1' => 1,
                        _ => 0,
                    }
                })
                .fold(0, |acc, x| acc + x);
            let co2_sum = co2_filtered
                .iter()
                .map(|s| {
                    let char = s.as_bytes()[i] as char;
                    match char {
                        '0' => 1,
                        '1' => 0,
                        _ => 0,
                    }
                })
                .fold(0, |acc, x| acc + x);

            if oxy_sum * 2 >= oxy_filtered.len() {
                oxy_str.push('1');
            } else {
                oxy_str.push('0');
            }

            if co2_sum * 2 > co2_filtered.len() {
                co2_str.push('1');
            } else {
                co2_str.push('0');
            }

            oxy_filtered = oxy_filtered
                .iter()
                .filter(|p| p.starts_with(oxy_str.as_str()))
                .map(|s| *s)
                .collect::<Vec<&str>>();
            co2_filtered = co2_filtered
                .iter()
                .filter(|p| p.starts_with(co2_str.as_str()))
                .map(|s| *s)
                .collect::<Vec<&str>>();

            if oxy_filtered.len() == 1 {
                oxygen = isize::from_str_radix(oxy_filtered[0], 2).unwrap();
            }

            if co2_filtered.len() == 1 {
                co2 = isize::from_str_radix(co2_filtered[0], 2).unwrap();
            }
        }
        println!("{}, {}", oxygen, co2);
        return (oxygen * co2).to_string();
    }
}
