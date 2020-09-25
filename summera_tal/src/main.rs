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

    // get input lines as strings
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap().to_string())
        .collect::<Vec<String>>();

    /* add code here ... */

    eprintln!("Kattis skips this comment!");
    //println!("Print to standard output.");
}