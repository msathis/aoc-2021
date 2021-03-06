pub(crate) mod day1;
pub(crate) mod day10;
pub(crate) mod day2;
pub(crate) mod day3;
pub(crate) mod day4;
pub(crate) mod day5;
pub(crate) mod day6;
pub(crate) mod day7;
pub(crate) mod day8;
pub(crate) mod day9;
pub(crate) mod problem;

// use std::fs;
// use std::path::Path;
// use std::vec::Vec;
//
// extern crate proc_macro;
// use proc_macro::TokenStream;
//
// struct Day {
//     mod_name: String,
//     num: u8,
// }
//
// impl Day {
//     fn new(file: &Path) -> Result<Day, ()> {
//         let day_file_pattern = Regex::new(r"^day(\d{2})\.rs$").unwrap();
//         let basename = file.file_name().ok_or(())?.to_str().ok_or(())?;
//
//         let day = Day {
//             // filename: file.to_path_buf(),
//             mod_name: String::from(&basename[..basename.len() - 3]),
//             num: day_file_pattern
//                 .captures(basename)
//                 .ok_or(())?
//                 .get(1)
//                 .ok_or(())?
//                 .as_str()
//                 .parse()
//                 .map_err(|_| ())?,
//         };
//         Ok(day)
//     }
//
//     fn vec_from_dir(dir: &str) -> Result<Vec<Day>, ()> {
//         let days: Vec<Day> = fs::read_dir(dir)
//             .map_err(|_| ())?
//             .map(|entry| Day::new(&entry.map_err(|_| ())?.path()))
//             .filter(|day_maybe| day_maybe.is_ok())
//             .map(|day_maybe| {
//                 let day = day_maybe.unwrap();
//                 day
//             })
//             .collect();
//         Ok(days)
//     }
// }
//
// #[proc_macro]
// pub fn import_advent(_: TokenStream) -> TokenStream {
//     let days = Day::vec_from_dir("advent").unwrap();
//     let code: String = format!(
//         "mod advent
// {{
// {}
// }}",
//         days.iter()
//             .map(|day| format!("    pub mod {};", day.mod_name))
//             .collect::<Vec<String>>()
//             .join("\n")
//     );
//
//     println!("Days imported with\n\n{}\n", code);
//
//     code.parse().unwrap()
// }
//
// fn match_part(day: &Day, part: &str, input: &str) -> String {
//     format!(
//         "match {part}
//     {{
//         1 => advent::{day}::part1({input}).to_string(),
//         2 => advent::{day}::part2({input}).to_string(),
//         _ => panic!(\"There's no part {{}}.\", part),
//     }}",
//         day = day.mod_name,
//         part = part,
//         input = input
//     )
// }
//
// #[proc_macro]
// pub fn define_run(item: TokenStream) -> TokenStream {
//     let mut iter = item.into_iter();
//     let day_var = iter.next().unwrap().to_string();
//     let part_var = iter.next().unwrap().to_string();
//     let input_var = iter.next().unwrap().to_string();
//
//     let days = Day::vec_from_dir("advent").unwrap();
//     let code: String = format!(
//         "fn run({day}: u8, {part}: u8, {input}: &str) -> String
// {{
// match {day}
// {{
// {body},
//     _ => panic!(\"There's no day {{}}.\", {day}),
// }}
// }}",
//         day = day_var,
//         part = part_var,
//         input = input_var,
//         body = days
//             .iter()
//             .map(|day| format!(
//                 "    {} => {}",
//                 day.num,
//                 match_part(&day, &part_var[..], &input_var[..])
//             ))
//             .collect::<Vec<String>>()
//             .join(",\n")
//     );
//
//     println!("The run() function is defined as\n\n{}\n", code);
//     code.parse().unwrap()
// }
