use crate::utils::{extract_between, extract_end};
use super::utils;
#[allow(dead_code)]
pub fn one() {
    println!("Day four, part one");
    let _content = super::reader::read_file("./input/day_four.txt");

    let lines: Vec<String> = _content.split("\n").map(|s| s.to_string()).collect();
    let mut total = 0;
    for line in lines {
        let mut _pointsCard = 0;
        let winners = extract_between(&line, ':', '|');
        let mine = extract_end(&line, '|').unwrap()
            .split_whitespace();
        let winNumbers : std::collections::HashMap<usize, bool> = winners.unwrap()
            .split_whitespace()
            .enumerate()
            .map(|(_, n)| (n.parse().unwrap(), true))
            .collect();

        // println!("Winners: {:?}", winNumbers);
        for m in mine {
            let n = m.parse::<usize>().unwrap();
            // println!("Number: {:?}", n);
            if winNumbers.contains_key(&n) {
                // println!("Found: {:?}", n);
                // println!("Points card: {:?}", _pointsCard);
                if _pointsCard > 0 {
                    _pointsCard = _pointsCard * 2;
                    continue;
                }
                _pointsCard = 1;
            }
        }
        total += _pointsCard;
        println!("_pointsCard: {:?}", _pointsCard);
    }
    println!("Total is: {:?}", total);
}

#[allow(dead_code)]
pub fn two() {
    println!("Day four, part one");
    let _content = super::reader::read_file("./input/day_four.txt");

    let lines: Vec<String> = _content.split("\n").map(|s| s.to_string()).collect();

    let mut _copies: Vec<i32> = vec![0; lines.len()];
    let mut scratchedCards = 0;

    for (i,line) in lines.iter().enumerate() {
        scratchedCards += 1;
        println!("card: {:?} plus {} copies", i+1, _copies[i]);

        let mut wins = 0;
        let winners = extract_between(&line, ':', '|');
        let mine = extract_end(&line, '|').unwrap()
            .split_whitespace();
        let winNumbers : std::collections::HashMap<usize, bool> = winners.unwrap()
            .split_whitespace()
            .enumerate()
            .map(|(_, n)| (n.parse().unwrap(), true))
            .collect();

        // println!("Winners: {:?}", winNumbers);
        for m in mine {
            let n = m.parse::<usize>().unwrap();
            if winNumbers.contains_key(&n) {
                wins += 1;
            }
        }

        let mut plus = 1;
        if _copies[i] > 0 {
            // _pointsCard += _pointsCard * _copies[i];
            scratchedCards += _copies[i];
            plus += _copies[i];
        }

        let mut copy = i + 1;
        while copy < _copies.len() && wins > 0 {
            _copies[copy] += plus;
            wins -= 1;
            copy += 1;
        }
    }
    println!("Total is: {:?}", scratchedCards);
}
