/***
 * Solution to the Kattis problem "Summera tal"
 * See: https://kth.kattis.com/problems/kth.javap.sumsort
 * Author: Mikael Sahleström <sahlest@kth.se>
 * Based on kattis_template
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();
   
    // ----------------------------------------------------------------
    // Imported from T.A Viola 2020-10-01
    // get iterable of all input lines
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    // get first line as usize
    let n = lines 
        // get first line of input
        .next().unwrap()
        // assuming convertability, convert to unsigned integer   
        .parse::<usize>().unwrap();
        
    // get second line as vector of u64
    let mut nums = lines
        // get second line of input
        .next().unwrap()
        // get iterator of substrings (optimized version of split(" "))
        .split_whitespace()
        // for every substring, assuming convertability, convert to unsigned integer
        .map(|_num| _num.parse::<u64>().unwrap())
        // assuming you acctually want a vector
        .collect::<Vec<u64>>();
    // ----------------------------------------------------------------

    // Ineffective method, since we iterate over each element multiple times
    nums.sort();
    nums.reverse();

    //Check how many numbers are required based on even or uneven n
    let m = if n%2 == 1 {(n+1)/2} else {n/2};
    
    // Sum the highest input vector
    let mut sum = 0;
    for i in 0..m {
        sum += nums[i];
    }
    println!("{}", sum);
}