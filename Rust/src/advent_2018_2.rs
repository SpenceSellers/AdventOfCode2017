use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use itertools::Itertools;

extern crate itertools;

fn read_input() -> Vec<String> {
    let mut file = File::open("input.txt").expect("Could not find input file");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("Could not read file");

    return string.lines()
        .map(|x| x.to_string())
        .collect();
}

fn histogram<T>(items: &[T]) -> HashMap<&T, usize>
    where T: Hash + Eq + Debug {
    let mut counts = HashMap::new();
    for item in items {
        counts.entry(item).and_modify(|e| *e += 1).or_insert(1);
    }
    return counts;
}

fn has_entries_that_appear_n_times<T: Hash + Eq>(hist: &HashMap<&T, usize>, n: usize) -> bool {
    hist.iter().any(|(_key, val)| *val == n)
}

fn part_1() {
    let lines = read_input();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let histograms: Vec<HashMap<&char, usize>> = chars.iter().map(|l| histogram(l)).collect();

    let num_of_2 = histograms.iter()
        .filter(|hist| has_entries_that_appear_n_times(hist, 2))
        .count();
    let num_of_3 = histograms.iter()
        .filter(|hist| has_entries_that_appear_n_times(hist, 3))
        .count();

    println!("{} * {} = {}", num_of_2, num_of_3, num_of_2 * num_of_3);
}

fn dissimilar_chars(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).filter(|(aa, bb)| aa != bb).count()
}

fn part_2() {
    let lines = read_input();
    let chars: Vec<Vec<char>> = lines.into_iter().map(|l| l.chars().collect()).collect();
    let (a,b)  = chars.into_iter()
        .combinations(2)
        .map(|combo| (combo[0].clone(), combo[1].clone()))
        .find(|(a, b)| dissimilar_chars(&a, &b) == 1)
        .expect("There are no boxes that differ by a single character");

    let a_string: String = a.iter().collect();
    let b_string: String = b.iter().collect();
    println!("{}", a_string);
    println!("{}", b_string);
}

fn main() {
    part_1();
    part_2();
}
