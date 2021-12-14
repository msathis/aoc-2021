use crate::Problem;
use std::collections::HashSet;

pub struct Day9 {}

impl Problem for Day9 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day9/input.txt".to_owned());
        let data = data
            .lines()
            .map(|l| {
                return l
                    .split("")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            })
            .collect::<Vec<Vec<i32>>>();

        let mut sum = 0;
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                let mut low = Self::is_low(i as i32, j as i32 - 1, &data, data[i][j]);
                low = low && Self::is_low(i as i32 - 1, j as i32, &data, data[i][j]);
                low = low && Self::is_low(i as i32 + 1, j as i32, &data, data[i][j]);
                low = low && Self::is_low(i as i32, j as i32 + 1, &data, data[i][j]);

                if low {
                    sum += data[i][j] + 1;
                }
            }
        }

        sum.to_string()
    }

    fn part_two(&self) -> String {
        let data = self.input_as_string("src/aoc/day9/input.txt".to_owned());
        let data = data
            .lines()
            .map(|l| {
                return l
                    .split("")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            })
            .collect::<Vec<Vec<i32>>>();

        let mut sums = vec![];
        let mut set = HashSet::new();
        for i in 0..data.len() {
            for j in 0..data[i].len() {
                let size = self.find_basin_size(i as i32, j as i32, &data, &mut set);
                sums.push(size);
            }
        }

        sums.sort();
        sums.iter()
            .rev()
            .take(3)
            .fold(1, |acc, x| acc * x)
            .to_string()
    }
}

impl Day9 {
    fn is_low(i: i32, j: i32, data: &Vec<Vec<i32>>, curr: i32) -> bool {
        if i < 0 || j < 0 || i >= data.len() as i32 || j >= data[i as usize].len() as i32 {
            return true;
        }
        curr < data[i as usize][j as usize]
    }

    fn find_basin_size(
        &self,
        i: i32,
        j: i32,
        data: &Vec<Vec<i32>>,
        set: &mut HashSet<String>,
    ) -> i32 {
        if i < 0
            || j < 0
            || i >= data.len() as i32
            || j >= data[i as usize].len() as i32
            || set.contains(format!("{}_{}", i, j).as_str())
            || &data[i as usize][j as usize] == &9
        {
            return 0;
        }

        let val = data[i as usize][j as usize];
        set.insert(format!("{}_{}", i, j));

        return 1
            + self.find_basin_size(i + 1, j, data, set)
            + self.find_basin_size(i - 1, j, data, set)
            + self.find_basin_size(i, j + 1, data, set)
            + self.find_basin_size(i, j - 1, data, set);
    }
}
