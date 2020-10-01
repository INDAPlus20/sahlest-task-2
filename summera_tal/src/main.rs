/***
 * Solution to the Kattis problem "Summera tal"
 * See: https://kth.kattis.com/problems/kth.javap.sumsort
 * Author: Mikael Sahleström <sahlest@kth.se>
 */

use std::io;
use std::io::prelude::*;

/// Kattis calls main function to run your solution.
fn main() {
    // get standard input stream
    let input = io::stdin();

    // get iterable of all input lines as String
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

    /* Dummy data ... */
    // let input1 = 5;
    // let mut input2 =vec![5, 3, 2, 1, 1]; // answer 10
    // let input1 = 9;
    // let mut input2 = vec![1, 14, 67, 83, 42, 6, 17, 33, 91]; // answer 316

    let input1 = n;
    let mut input2 = nums; // answer 316
    

    // Ineffective method, since we iterate over each element multiple times
    input2.sort();
    input2.reverse();

    //Check how many numbers are required 
    let m = if input1%2 == 1 {(input1+1)/2} else {input1/2};
    let mut index = 0;

    // Sum the highest input vector
    let mut sum = 0;
    while index < m {
        sum += input2[index];
        index += 1;
    }


    // Check if n is even or uneven to decide how many number to sum
    // eprintln!("{}", m);

    // Create vector with the length of m. 
    // let highest_input = vec![0; (input1+1)/2]; // always rounds down by default

    // Start work through the input vector. Is the current value higher than the lowest value in highestInput? then replace that value.alloc



    // let f: bool = false;
    // let maxValue = input2.iter().max();

    // eprintln!("Kattis skips this comment!");
    println!("{}", sum);
}