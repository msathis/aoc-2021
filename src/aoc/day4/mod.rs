use crate::Problem;
use std::collections::HashSet;

pub struct Day4 {}

impl Problem for Day4 {
    fn part_one(&self) -> String {
        let file = self.input_as_string("src/aoc/day4/input.txt".to_owned());
        let mut lines = file.lines();
        let random: Vec<i32> = lines
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let rows: Vec<Vec<i32>> = lines
            .map(|s| {
                return s
                    .split(" ")
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
            })
            .filter(|v| v.len() > 0)
            .collect();

        let mut hashes = vec![];
        let mut boards = vec![];

        for x in 0..(rows.len() / 5) {
            let mut board_hash = vec![HashSet::new(); 10];
            let mut board = vec![vec![0; 5]; 5];
            for i in 0..5 {
                for j in 0..5 {
                    let val = rows[(x * 5) + i][j];
                    board_hash[i].insert(val);
                    board_hash[i + 5].insert(rows[(x * 5) + j][i]);
                    board[i][j] = val;
                }
            }
            boards.push(board);
            hashes.push(board_hash);
        }

        let res = Self::find_board(&random, &mut hashes);
        let total_unmarked = hashes[res.1]
            .iter()
            .skip(5)
            .map(|h| h.iter().fold(0, |acc, x| acc + x))
            .fold(0, |acc, x| acc + x);

        return (total_unmarked * res.0).to_string();
    }

    fn part_two(&self) -> String {
        todo!()
    }
}

impl Day4 {
    fn find_board(random: &Vec<i32>, hash: &mut Vec<Vec<HashSet<i32>>>) -> (i32, usize) {
        for num in random {
            for (i, h) in hash.iter_mut().enumerate() {
                for x in h {
                    x.remove(num);
                    if x.len() == 0 {
                        return (*num, i);
                    }
                }
            }
        }
        return (-1, 1);
    }
}
