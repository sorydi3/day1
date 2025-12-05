use core::panic;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
fn main() -> () {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file);
    let mut res_counter = 0;
    reader.lines().fold(50i32, |acc, line| match line {
        Ok(line) => {
            let command = line.chars().collect::<Vec<_>>();
            let tics = command.iter().skip(1).collect::<String>();
            let tics = tics.parse::<i32>().unwrap();
            let res = match command[0] {
                'L' => (acc - tics) % 100,
                'R' => (acc + tics) % 100,
                _ => panic!("NOT SUPPORTED"),
            };

            if res == 0 {
                res_counter += 1;
            }
            res
        }
        _ => acc,
    });
    println!("RES_COUNTER: {:?}", res_counter);
}
