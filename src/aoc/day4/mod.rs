use crate::Problem;
use std::collections::HashSet;
use std::str::Lines;

pub struct Day4 {}

impl Problem for Day4 {
    fn part_one(&self) -> String {
        let file = self.input_as_string("src/aoc/day4/input.txt".to_owned());
        let mut lines = file.lines();
        let (random, rows) = Self::parse_input(&mut lines);
        let mut hashes = Self::prepare_boards(rows);

        let res = Self::find_board(&random, &mut hashes);
        let total_unmarked = Self::find_total_unmarked(&mut hashes, res.1);

        return (total_unmarked * res.0).to_string();
    }

    fn part_two(&self) -> String {
        let file = self.input_as_string("src/aoc/day4/input.txt".to_owned());
        let mut lines = file.lines();
        let (random, rows) = Self::parse_input(&mut lines);
        let mut hashes = Self::prepare_boards(rows);

        let res = Self::find_last_board(&random, &mut hashes);
        let total_unmarked = Self::find_total_unmarked(&mut hashes, res.1);

        return (total_unmarked * res.0).to_string();
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
        return (-1, 0);
    }

    fn find_last_board(random: &Vec<i32>, hash: &mut Vec<Vec<HashSet<i32>>>) -> (i32, usize) {
        let mut board_checked = HashSet::new();
        let size = hash.len();
        for num in random {
            for (i, h) in hash.iter_mut().enumerate() {
                for x in h {
                    x.remove(num);
                    if x.len() == 0 {
                        board_checked.insert(i);
                    }

                    if board_checked.len() == size {
                        let mut vec = board_checked.into_iter().collect::<Vec<_>>();
                        return (*num, i);
                    }
                }
            }
        }
        return (-1, 0);
    }

    fn parse_input(lines: &mut Lines) -> (Vec<i32>, Vec<Vec<i32>>) {
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
        (random, rows)
    }

    fn prepare_boards(rows: Vec<Vec<i32>>) -> Vec<Vec<HashSet<i32>>> {
        let mut hashes = vec![];

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
            hashes.push(board_hash);
        }
        hashes
    }

    fn find_total_unmarked(hashes: &mut Vec<Vec<HashSet<i32>>>, index: usize) -> i32 {
        let total_unmarked = hashes[index]
            .iter()
            .skip(5)
            .map(|h| h.iter().fold(0, |acc, x| acc + x))
            .fold(0, |acc, x| acc + x);
        total_unmarked
    }
}
