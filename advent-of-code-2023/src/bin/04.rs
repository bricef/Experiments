

use std::collections::HashSet;

use advent_of_code_2023::libaoc::read_lines;





#[derive(Debug)]
struct Card {
    // id: u32,
    // numbers: Vec<u32>,
    // winners: Vec<u32>,
    matches: u32,
    points: u32,
}

fn points_from_matches(m:u32) -> u32{
    match m {
        0 => 0,
        1 => 1,
        _ => 2*points_from_matches(m-1)
    }
}

impl Card {
    fn from_line(line: &String) -> Card {
        let parts : Vec<&str> = line.split(":").collect();
        // let id = parts[0][4..].trim().parse::<u32>().unwrap();
        let xs : Vec<&str> = parts[1].split("|").collect();
        let numbers: Vec<u32> = xs[0].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let winners: Vec<u32> = xs[1].split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        let n_set: HashSet<u32> = HashSet::from_iter(numbers.clone());
        let w_set: HashSet<u32> = HashSet::from_iter(winners.clone());
        let intersection: HashSet<_> = n_set.intersection(&w_set).collect();
        Card {
            // id: id,
            // numbers: numbers,
            // winners: winners,
            matches: intersection.len() as u32,
            points: points_from_matches(intersection.len() as u32)
        }

    }
}

fn total_points(cards: &Vec<Card>)  -> u32 {
    let points = cards.iter()
        .map(|c| c.points)
        .fold(0, |acc, n| acc+n);
    return points;
}

fn card_count(cards: &Vec<Card>) -> u32 {
    let mut acc: usize = 0;
    let mut counts = vec![1; cards.len()];
    // println!("{:?}", counts);
    for (i, card) in cards.iter().enumerate() {
        acc += counts[i];
        for x in 0..card.matches {
            let ix: usize = i+1+(x as usize);
            counts[ix] += counts[i];
        }
    }
    // println!("{:?}", counts);
    return acc as u32 ;
}

fn main () {
    let example = read_lines("files/04-example.txt");
    let input = read_lines("files/04-input.txt");

    let example_cards: Vec<Card> = example.iter().map(|l| Card::from_line(l)).collect();
    let input_cards: Vec<Card> = input.iter().map(|l| Card::from_line(l)).collect();
    
    let example_expected_points = 13;
    let example_points = total_points(&example_cards);
    let input_points = total_points(&input_cards);
    
    println!("# Day 04");
    
    println!("## Part 1");
    println!("Example Points: {:?} (expected {})", example_points, example_expected_points);
    println!("Input Points: {}", input_points);

    let example_expected_card_count = 30;
    let example_card_count = card_count(&example_cards);
    let input_card_count = card_count(&input_cards);

    println!("## Part 2");
    println!("Example total card count: {} (expected {})", example_card_count, example_expected_card_count);
    println!("Input total card count: {}", input_card_count);




}