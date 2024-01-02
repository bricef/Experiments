use std::{cmp::Ordering, collections::HashMap};

use advent_of_code_2023::libaoc::read_lines;
use itertools::Itertools;

/*

CAMEL CARDS

In Camel Cards, you get a list of hands, and your goal is to order them based 
on the strength of each hand. A hand consists of five cards labeled one of 
A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2. The relative strength of each card 
follows this order, where A is the highest and 2 is the lowest.

Every hand is exactly one type. From strongest to weakest, they are:

- Five of a kind, where all five cards have the same label: AAAAA
- Four of a kind, where four cards have the same label and one card has a 
  different label: AA8AA
- Full house, where three cards have the same label, and the remaining two 
  cards share a different label: 23332
- Three of a kind, where three cards have the same label, and the remaining 
  two cards are each different from any other card in the hand: TTT98
- Two pair, where two cards share one label, two other cards share a second 
  label, and the remaining card has a third label: 23432
- One pair, where two cards share one label, and the other three cards have 
  a different label from the pair and each other: A23A4
- High card, where all cards' labels are distinct: 23456

Hands are primarily ordered based on type; for example, every full house is 
stronger than any three of a kind.

If two hands have the same type, a second ordering rule takes effect. Start 
by comparing the first card in each hand. If these cards are different, the 
hand with the stronger first card is considered stronger. If the first card 
in each hand have the same label, however, then move on to considering the 
second card in each hand. If they differ, the hand with the higher second 
card wins; otherwise, continue with the third card in each hand, then the 
fourth, then the fifth.

So, 33332 and 2AAAA are both four of a kind hands, but 33332 is stronger 
because its first card is stronger. Similarly, 77888 and 77788 are both a 
full house, but 77888 is stronger because its third card is stronger (and 
both hands have the same first and second card).
*/


#[derive(Debug)]
struct CamelHand {
    cards: String,
    bid: u32,
    hand_type: HandType
}

#[derive(Debug, Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPairs = 3,
    OnePair = 2,
    HighCard = 1
}

fn to_char_count(input: &str) -> HashMap<char,u32>{
    input
        .to_uppercase()
        .chars()
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

fn card_to_val(c: char) -> u32 {
    match c {
        'A'=> 14, 
        'K' => 13, 
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9, 
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unknown Card type {:?}", c)
    }
}

fn card_to_val_jokers_weak(c: char) -> u32 {
    match c {
        'J' => 1,
        _ => card_to_val(c)
    }
}

fn hand_type_of(cards: &str) -> HandType {
    let charcounts = to_char_count(cards);
    let max_count = *charcounts.values().max().unwrap();
    if max_count == 5 { return HandType::FiveOfAKind }
    if max_count == 4  { return HandType::FourOfAKind }
    let second_count = *charcounts.values().sorted().rev().collect_vec()[1];
    // println!("CARDS {}. {:?}, first: {}, Second: {}, {:?}", 
    //         cards, charcounts, max_count, second_count, charcounts.values().sorted().rev().collect_vec());
    if max_count == 3 && second_count == 2 { return HandType::FullHouse }
    if max_count == 3 { return HandType::ThreeOfAKind; }
    if max_count == 2 && second_count == 2 {return HandType::TwoPairs; }
    if max_count == 2 { return HandType::OnePair; }
    return HandType::HighCard
}

fn hand_type_with_jokers_of(cards: &str) -> HandType {
    if cards.contains('J') {
        let mut charcounts = to_char_count(cards);
        println!("cards: {:?}, {:?}", cards, charcounts);
        let j_count = *charcounts.get(&'J').unwrap();
        if j_count == 5 { return HandType::FiveOfAKind }
        charcounts.remove(&'J');
        let max_count = *charcounts.values().max().unwrap() + j_count;
        if max_count == 5 { return HandType::FiveOfAKind }
        if max_count == 4 { return HandType::FourOfAKind }        
        let second_count = *charcounts.values().sorted().rev().collect_vec()[1];
        if max_count == 3 && second_count == 2  { return HandType::FullHouse }
        if max_count == 3 { return HandType::ThreeOfAKind; }
        if max_count == 2 && second_count == 2 {return HandType::TwoPairs; }
        if max_count == 2 { return HandType::OnePair; }
        return HandType::HighCard
    }else {
        return hand_type_of(cards);
    }
}


impl CamelHand {
    fn from(line: &str) -> CamelHand {
        let segments = line.split_whitespace().collect_vec();
        let bid = segments[1].parse::<u32>().unwrap();
        let hand = CamelHand {
            cards: String::from(segments[0]),
            bid: bid,
            hand_type: hand_type_of(segments[0])
        };
        return hand
    }
    
}

struct JokerHand {
    cards: String,
    bid: u32,
    hand_type: HandType
}

impl JokerHand {
    fn from(line: &str) -> JokerHand {
        let segments = line.split_whitespace().collect_vec();
        let bid = segments[1].parse::<u32>().unwrap();
        let hand = JokerHand {
            cards: String::from(segments[0]),
            bid: bid,
            hand_type: hand_type_with_jokers_of(segments[0])
        };
        return hand
    }
}
impl PartialEq for JokerHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for JokerHand {}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = self.hand_type as isize;
        let oth = other.hand_type as isize;
        if me > oth { return Ordering::Greater}
        if me < oth { return Ordering::Less}
        for (me_c, oth_c) in self.cards.chars().zip(other.cards.chars()){
            if card_to_val_jokers_weak(me_c) > card_to_val_jokers_weak(oth_c) {return Ordering::Greater;}
            if card_to_val_jokers_weak(me_c) < card_to_val_jokers_weak(oth_c) {return Ordering::Less;}
        }
        return Ordering::Equal;
    }
}



impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for CamelHand {}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let me = self.hand_type as isize;
        let oth = other.hand_type as isize;
        if me > oth { return Ordering::Greater}
        if me < oth { return Ordering::Less}
        for (me_c, oth_c) in self.cards.chars().zip(other.cards.chars()){
            if card_to_val(me_c) > card_to_val(oth_c) {return Ordering::Greater;}
            if card_to_val(me_c) < card_to_val(oth_c) {return Ordering::Less;}
        }
        return Ordering::Equal;
    }
}

fn total_winnings(hands: &mut Vec<CamelHand>) ->u32 {
    hands.sort();
    let mut acc = 0;
    for (i, h) in hands.iter().enumerate() {
        acc += ((i as u32)+1)*h.bid;
    }
    return acc;
}

fn total_winnings_joker(hands: &mut Vec<JokerHand>) ->u32 {
    hands.sort();
    let mut acc = 0;
    for (i, h) in hands.iter().enumerate() {
        acc += ((i as u32)+1)*h.bid;
    }
    return acc;
}
fn main(){
    let mut example = read_lines("files/07-example.txt").iter().map(|l| CamelHand::from(l)).collect_vec();
    let mut input = read_lines("files/07-input.txt").iter().map(|l| CamelHand::from(l)).collect_vec();
    let example_total_expected_winning = 6440;

    let example_total_winning = total_winnings(&mut example);
    let input_total_winning = total_winnings(&mut input);

    println!("# Day 07");
    println!("## Part 1");
    println!("Example total winnings: {} (expected {})", example_total_winning, example_total_expected_winning);
    println!("Input total winnings: {}", input_total_winning);

    let mut joker_example = read_lines("files/07-example.txt").iter().map(|l| JokerHand::from(l)).collect_vec();
    let mut joker_input = read_lines("files/07-input.txt").iter().map(|l| JokerHand::from(l)).collect_vec();
    let example_total_joker_winning = total_winnings_joker(&mut joker_example);
    let input_total_joker_winning = total_winnings_joker(&mut joker_input);
    println!("## Part 2");
    println!("Joker Example total winnings: {} (expected 5905)", example_total_joker_winning);
    println!("Joker Input total winnings: {}", input_total_joker_winning);
   
    

}