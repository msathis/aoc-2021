use crate::Problem;

pub struct Day8 {}

impl Problem for Day8 {
    fn part_one(&self) -> String {
        let data = self.input_as_string("src/aoc/day8/input.txt".to_owned());
        let count = data
            .lines()
            .map(|x| x.split(" ").skip(11).collect::<Vec<&str>>())
            .flatten()
            .filter(|x| return x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
        return count.to_string();
    }

    fn part_two(&self) -> String {
        todo!()
    }
}
