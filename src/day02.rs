use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Day02 {
    reader: BufReader<File>,
}

trait IsSafeLevel {
    fn is_safe(&self) -> bool;
}

#[derive(Debug, PartialEq)]
enum LevelTrend {
    Increasing,
    Decreasing,
    NotSet,
}

impl IsSafeLevel for Vec<u32> {
    fn is_safe(&self) -> bool {
        let mut retries = 0;
        let max_retries = 1;
        let mut stack: Vec<(u32, LevelTrend)> = vec![];

        for num in self.iter() {
            if stack.is_empty() {
                stack.push((*num, LevelTrend::NotSet));
                continue;
            }

            let (prev_elem, prev_trend) = stack.pop().unwrap();

            if *num == prev_elem || num.abs_diff(prev_elem) > 3 {
                if retries < max_retries {
                    retries += 1;
                    stack.push((*num, prev_trend));
                    continue;
                }
                return false;
            }

            let trend = if *num > prev_elem {
                LevelTrend::Increasing
            } else {
                LevelTrend::Decreasing
            };

            match prev_trend {
                LevelTrend::Increasing => {
                    if trend == LevelTrend::Decreasing {
                        if retries < max_retries {
                            retries += 1;
                            stack.push((*num, prev_trend));
                            continue;
                        }
                        return false;
                    } else if trend == LevelTrend::Increasing {
                        stack.push((*num, trend));
                    }
                }
                LevelTrend::Decreasing => {
                    if trend == LevelTrend::Increasing {
                        if retries < max_retries {
                            retries += 1;
                            stack.push((*num, prev_trend));
                            continue;
                        }
                        return false;
                    } else if trend == LevelTrend::Decreasing {
                        stack.push((*num, trend));
                    }
                }
                LevelTrend::NotSet => {
                    stack.push((*num, trend));
                }
            }
        }

        true
    }
}

impl Day02 {
    pub fn new() -> Self {
        let f = File::open("testdata/day02").expect("file not found");
        let reader = BufReader::new(f);
        Self { reader }
    }

    pub fn run(&mut self) {
        let mut count = 0_u32;
        loop {
            let mut line = String::new();
            let bytes = self.reader.read_line(&mut line).unwrap();
            if bytes == 0 {
                break;
            }
            let levels = Self::extract(&line);
            if levels.is_safe() {
                count += 1;
            }
        }

        println!("(day02) Safe levels: {}", count);
    }

    fn extract(input: &str) -> Vec<u32> {
        input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
    }
}
