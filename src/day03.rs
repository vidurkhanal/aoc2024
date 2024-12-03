use regex::Regex;
use std::fs;

pub fn run() {
    // let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let test_input = fs::read_to_string("testdata/day03").unwrap();
    let re = Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut mul_enabled = true;
    let result: i32 = re
        .captures_iter(&test_input)
        .map(|caps| {
            return match caps.get(1) {
                Some(_) => {
                    let multiplicand: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
                    let multiplier: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
                    if mul_enabled {
                        multiplicand * multiplier
                    } else {
                        0
                    }
                }
                None => {
                    match caps.get(0).unwrap().as_str() {
                        "do()" => mul_enabled = true,
                        "don't()" => mul_enabled = false,
                        _ => unreachable!(),
                    }
                    0
                }
            };
        })
        .sum();

    println!("Result: {result}")
}
