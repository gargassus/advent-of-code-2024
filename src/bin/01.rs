use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i128> {

        // start by getting two lists from input
        let mut left_list: Vec<i128>  = vec![];
        let mut right_list: Vec<i128>  = vec![];

        // read lists
        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(2, words.len());
            
            //debug
            //println!("{}, {}", words[0], words[1]);
            
            left_list.push(words[0].parse().unwrap());
            right_list.push(words[1].parse().unwrap());
        }

        // sort lists
        left_list.sort();
        right_list.sort();

        // debug
        //println!("{:?}", left_list);
        //println!("{:?}", right_list);

        let mut answer: i128 = 0;

        // iterate lists, subtracting r value from l, getting posive out and summing it all up 
        for it in left_list.iter_mut().zip(right_list.iter_mut()) {
            let (l, r) = it;

            answer += (*l - *r).abs();
        }

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result: i128 = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<i128> {

        // start by getting two lists from input
        let mut left_list: Vec<i128>  = vec![];
        let mut right_list: Vec<i128>  = vec![];

        // read lists
        for line in reader.lines() {
            let line = line?;
            let words: Vec<&str> = line.split_whitespace().collect();
            assert_eq!(2, words.len());
            
            //debug
            //println!("{}, {}", words[0], words[1]);
            
            left_list.push(words.index(0).parse().unwrap());
            right_list.push(words.index(1).parse().unwrap());
        }

        let mut answer: i128 = 0;

        // iterate lists, subtracting r value from l, getting posive out and summing it all up 
        for i in left_list.iter() {

            let temp = i * right_list.iter().filter(|&n| *n == *i).count() as i128;

            answer += temp;
        }

        Ok(answer)
    }
    
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
