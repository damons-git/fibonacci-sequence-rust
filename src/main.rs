extern crate clap;
use clap::{Arg, App};

/**
 * Fibonacci Sequence generator
 *
 * This program generates the fibonacci sequence to a given limit.
 */

fn main() {
    let matches = App::new("Fibonacci Sequence")
        .version("0.0.1")
        .author("damons-git")
        .about("A simple application that generates the fibonacci sequence to a given depth.")
        .arg(Arg::with_name("DEPTH")
            .short("d")
            .long("depth")
            .takes_value(true)
            .help("Integer depth to generate the fibonacci sequence to"))
        .get_matches();
    let depth: i32 = (matches.value_of("DEPTH").unwrap_or("25")).parse::<i32>().unwrap();

    let sequence = fibonacci(depth);
    println!("{:?}", sequence);
}

fn fibonacci(mut depth: i32) -> Vec<i64> {
    if depth < 0 { panic!("Cannot generate a sequence of negative length.") }
    if depth == 0 { return vec![]; }
    if depth == 1 { return vec![0]; }
    if depth == 2 { return vec![0, 1]; }

    let mut acc: Vec<i64> = vec![0, 1];
    depth = depth - 2;

    while depth > 0 {
        let idx = acc.len() - 1;
        let new = acc[idx as usize] + acc[idx as usize - 1];
        acc.push(new);
        depth -= 1;
    }

    return acc;
}

// Module unit tests.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fibonacci_long() {
        let res: Vec<i64> = fibonacci(20);
        let expected: Vec<i64> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181];
        assert_eq!(res, expected);
    }

    #[test]
    fn fibonacci_short() {
        let res: Vec<i64> = fibonacci(5);
        let expected: Vec<i64> = vec![0, 1, 1, 2, 3];
        assert_eq!(res, expected);
    }

    #[test]
    fn fibonacci_edge_case_low() {
        let zero: Vec<i64> = fibonacci(0);
        let one: Vec<i64> = fibonacci(1);
        let two: Vec<i64> = fibonacci(2);
        println!("{:?}, {:?}, {:?}", zero, one, two);
        assert_eq!(zero, vec![]);
        assert_eq!(one, vec![0]);
        assert_eq!(two, vec![0, 1]);
    }

    #[test]
    #[should_panic]
    fn fibonacci_edge_case_high() {
        let _res: Vec<i64> = fibonacci(94);
    }

    #[test]
    #[should_panic]
    fn fibonacci_erroneous() {
        let _res: Vec<i64> = fibonacci(-1);
    }
}