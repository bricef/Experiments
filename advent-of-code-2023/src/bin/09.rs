

use advent_of_code_2023::libaoc::read_lines;


fn line_to_sequence(line: &str) -> Vec<i64>{
    line.split_whitespace().map(|d| d.parse::<i64>().unwrap()).collect()
}

fn parse_sequences(filename: &str) -> Vec<Vec<i64>> {
    read_lines(filename).iter()
        .map(|l| line_to_sequence(l))
        .collect()
}

fn seq_to_diff_seq(seq: &Vec<i64>) -> Vec<i64> {
    let mut out : Vec<i64> = Vec::with_capacity(seq.len()-1);
    for i in 0..seq.len()-1 {
        out.push(seq[i+1] - seq[i])
    }
    out
}

fn next_element(seq: &Vec<i64>) -> i64 {
    let mut new =  seq;
    let mut temp: Vec<i64> ;
    let mut ds:Vec<i64> = vec![];
    ds.push(*new.last().unwrap());
    loop {
        temp = seq_to_diff_seq(&new);
        // println!("{:?}", &temp);
        ds.push(*temp.last().unwrap());
        if temp.iter().all(|i| *i == 0){ break };
        new = &temp;
    }
    let next = ds.iter().sum::<i64>();
    return next;
}

fn sum_of_next_elements(seqs: &Vec<Vec<i64>>) -> i64 {
    seqs.iter().map(|seq| next_element(seq)).sum()
}

fn previous_element(seq: &Vec<i64>) -> i64 {
    let mut seqs: Vec<Vec<i64>> = vec![seq.clone()];
    loop {
        let latest = seq_to_diff_seq(seqs.last().unwrap());
        if (&latest).iter().all(|i| *i == 0){ 
            seqs.push(latest);
            break 
        };
        seqs.push(latest);
    }
    // println!("{:?}", seqs);
    let prev = seqs[..seqs.len()].iter().rev().fold(0, |acc, seq|{
        // println!("{:?}", seq);
        seq[0]-acc
    });
    return prev;
}

fn sum_of_previous_elements(seqs: &Vec<Vec<i64>>) -> i64 {
    seqs.iter().map(|seq: &Vec<i64>| previous_element(seq)).sum()
}

fn main () {
    let example: Vec<Vec<i64>> = parse_sequences("files/09-example.txt");
    let input: Vec<Vec<i64>> = parse_sequences("files/09-input.txt");

    println!("# Day 09");
    println!("# Part 1");
    
    println!("Example sum of next elements: {} (expect 114)", sum_of_next_elements(&example));
    println!("Input sum of next elements: {}", sum_of_next_elements(&input));

    println!("# Part 2");
    println!("Example previous term is {} (expected 5)", previous_element(&vec![10, 13, 16, 21, 30, 45]));
    println!("Sum of input previous terms {}", sum_of_previous_elements(&input));
    
}