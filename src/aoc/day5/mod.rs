use crate::Problem;
use std::cmp::{max, min};
use std::f32::consts::PI;
use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<usize> = s.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        Ok(Point {
            x: vec[0],
            y: vec[1],
        })
    }
}

pub struct Day5 {}

impl Problem for Day5 {
    fn part_one(&self) -> String {
        let file = self.input_as_string("src/aoc/day5/input.txt".to_owned());
        let lines: Vec<Line> = file
            .lines()
            .map(|x| {
                let points = x.split(" -> ").collect::<Vec<&str>>();
                let p1 = points[0].parse::<Point>().unwrap();
                let p2 = points[1].parse::<Point>().unwrap();
                return Line { start: p1, end: p2 };
            })
            .filter(|p| p.start.x == p.end.x || p.start.y == p.end.y)
            .collect();

        let mut points = vec![vec![0; 1000]; 1000];
        for line in lines {
            if line.start.x == line.end.x {
                let min_y = min(line.start.y, line.end.y);
                let max_y = max(line.start.y, line.end.y);

                for y in min_y..=max_y {
                    points[line.start.x][y] += 1;
                }
            } else {
                let min_x = min(line.start.x, line.end.x);
                let max_x = max(line.start.x, line.end.x);

                for x in min_x..=max_x {
                    points[x][line.start.y] += 1;
                }
            }
        }

        points
            .iter()
            .flatten()
            .filter(|p| p > &&1_i32)
            .count()
            .to_string()
    }

    fn part_two(&self) -> String {
        let file = self.input_as_string("src/aoc/day5/input.txt".to_owned());
        let lines: Vec<Line> = file
            .lines()
            .map(|x| {
                let points = x.split(" -> ").collect::<Vec<&str>>();
                let p1 = points[0].parse::<Point>().unwrap();
                let p2 = points[1].parse::<Point>().unwrap();
                return Line { start: p1, end: p2 };
            })
            .collect();

        let mut points = vec![vec![0; 1000]; 1000];
        for line in lines {
            let x_diff = (line.start.x as i32 - line.end.x as i32) as f32;
            let y_diff = (line.start.y as i32 - line.end.y as i32) as f32;
            let angle = x_diff.atan2(y_diff) * 180_f32 / PI;

            if line.start.x == line.end.x {
                let min_y = min(line.start.y, line.end.y);
                let max_y = max(line.start.y, line.end.y);

                for y in min_y..=max_y {
                    points[y][line.start.x] += 1;
                }
            } else if line.start.y == line.end.y {
                let min_x = min(line.start.x, line.end.x);
                let max_x = max(line.start.x, line.end.x);

                for x in min_x..=max_x {
                    points[line.start.y][x] += 1;
                }
            } else if angle.abs() == 45_f32 || angle.abs() == 135_f32 {
                let x_step: i32 = if line.start.x > line.end.x { -1 } else { 1 };
                let y_step: i32 = if line.start.y > line.end.y { -1 } else { 1 };
                let mut x = line.start.x as i32;
                let mut y = line.start.y as i32;

                while x != line.end.x as i32 && y != line.end.y as i32 {
                    points[y as usize][x as usize] += 1;
                    x += x_step;
                    y += y_step;
                }
                points[line.end.y][line.end.x] += 1;
            }
        }

        points
            .iter()
            .flatten()
            .filter(|p| p > &&1_i32)
            .count()
            .to_string()
    }
}
