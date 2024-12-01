use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub struct Day01 {
    reader: BufReader<File>,
    heap1: BinaryHeap<u32>,
    heap2: BinaryHeap<u32>,
    frequency_map: HashMap<u32, u32>,
}

impl Day01 {
    pub fn new() -> Self {
        let f = File::open("testdata/day01").expect("file not found");
        let reader = BufReader::new(f);
        Self {
            reader,
            heap1: BinaryHeap::new(),
            heap2: BinaryHeap::new(),
            frequency_map: HashMap::new(),
        }
    }

    fn populate(&mut self) {
        loop {
            let mut line = String::new();
            let bytes = self.reader.read_line(&mut line).unwrap();
            if bytes == 0 {
                break;
            }
            let numbers = Self::extract(&line);
            self.heap1.push(numbers[0]);
            self.heap2.push(numbers[1]);
            self.frequency_map
                .entry(numbers[1])
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }

    pub fn run(&mut self) {
        self.populate();

        let mut sum = 0_u32;
        let mut similarity_score = 0_u32;

        while !self.heap1.is_empty() && !self.heap2.is_empty() {
            let num1 = self.heap1.pop().unwrap();
            let num2 = self.heap2.pop().unwrap();

            let f = match self.frequency_map.get(&num1) {
                Some(f) => *f,
                None => 0,
            };

            similarity_score += num1 * f;

            sum += num1.abs_diff(num2);
        }

        println!("Day 01 (step1): {}", sum);
        println!("Day 01 (step2): {}", similarity_score);
    }

    fn extract(input: &str) -> Vec<u32> {
        input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect()
    }
}
