/***
 * Solution to the Kattis problem "A different problem"
 * See: https://kth.kattis.com/problems/kth.javap.sumsort
 * Author: Mikael Sahleström <sahlest@kth.se>
 * Based on kattis_template
 */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    // Imported from https://kth.kattis.com/help/rust
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let mut nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
             // Alt 1: Check which number is highest, example via sort, and then subtract on from the other
            nums.sort();
            let answer = nums[1]-nums[0]; 
            // Alt 2: Subtract on from the other, return the absolute value (might be problems due to the size of the numbe)
            // answer = (nums[0]-nums[1]).abs() 
            println!("{:?}", answer);
    }
}