#[allow(unused_imports)]
use std::{collections::HashMap, fs::read_to_string, thread::sleep, time};

use itertools::Itertools;
use regex::Regex;

const EXAMPLE_ONE: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

const EXAMPLE_TWO: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

const PART_TWO_EXAMPLE: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

type Graph = HashMap<String, (String, String)>;

type Instructions<'a> = &'a str;

#[allow(non_snake_case)]
fn L (choices: &(String, String)) -> &String {
    return &choices.0;
}

#[allow(non_snake_case)]
fn R(choices: &(String, String)) -> &String {
    return &choices.1;
}

fn extract_graph(def: &str) -> Graph {
    // println!("Extracting graph from: {}", def);
    let re = Regex::new(r#"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)"#).unwrap();
    let mut hm:HashMap<String, (String, String)> = HashMap::new();
    for (_, [from, left, right]) in re.captures_iter(def).map(|c| c.extract()) {
        hm.insert(String::from(from), (String::from(left), String::from(right)));
    }
    // println!("Generated map: {:?}", hm);
    return hm
}

fn extract_instructions(def: &str) -> Instructions {
    def
}

fn parse_definition(def: &str) -> (Graph, Instructions) {
    let parts = def.split("\n\n").collect_vec();
    let (instruction_def, graph_def) = (parts[0], parts[1]);
    return (extract_graph(graph_def), extract_instructions(instruction_def));
}

fn run(graph: &Graph, instructions: Instructions) -> u32 {
    let mut current = &String::from("AAA");
    let mut count: u32 = 0;
    for c in instructions.chars().cycle() {
        match c {
            'L' => {
                let options = graph.get(current).unwrap();
                current = L(options);
            },
            'R' => {
                let options = graph.get(current).unwrap();
                current = R(options);
            }
            _ => panic!("Unexpected instruction '{}'", c)
        }
        count += 1;
        if current == "ZZZ" { break }
    }
    count
}

fn run_parallel(graph: &Graph, instructions: Instructions) -> u64 {
    let mut count : u64 = 0;
    let mut positions = graph.keys().filter(|p| p.ends_with('A')).collect_vec();
    println!("Starting positions: {:?}", positions);
    for c in instructions.chars().cycle() {
        match c {
            'L' => {
                positions = positions.iter().map(|p| {
                    let options = graph.get(*p).unwrap();
                    return L(options)
                }).collect_vec();
            } 
            'R' => {
                positions = positions.iter().map(|p| {
                    let options = graph.get(*p).unwrap();
                    return R(options)
                }).collect_vec();
            }
            _ => panic!("Unexpected instruction '{}'", c)
        }
        count +=1;
        // println!("cycle {}, positions: {:?}", count, positions);
        // sleep(time::Duration::from_millis(200))
        if count % 100000 == 0 {
            println!("cycle {}, positions: {:?}", count, positions);
        }
        if positions.iter().all(|p| p.ends_with('Z')) { break }
        
    }
    count
}

fn main(){
    let input = read_to_string("files/08-input.txt").unwrap();
    let (example_one_graph, example_one_instructions) = parse_definition(EXAMPLE_ONE);
    let (example_two_graph, example_two_instructions) = parse_definition(EXAMPLE_TWO);
    let (part_two_example_graph, part_two_example_instructions) = parse_definition(PART_TWO_EXAMPLE);
    let (input_graph, input_instructions) = parse_definition(&input);

    println!("# Day 08");
    println!("## Part 1");

    let example_one_cycles = run(&example_one_graph, example_one_instructions);
    println!("Example one cycles: {} (expected 2)", example_one_cycles);

    let example_two_cycles = run(&example_two_graph, example_two_instructions);
    println!("Example two cycles: {} (expected 6)", example_two_cycles);

    let input_cycles = run(&input_graph, input_instructions);
    println!("Input cycles: {}", input_cycles);

    println!("## Part 2");
    let part_two_example_cycles = run_parallel(&part_two_example_graph, part_two_example_instructions);
    println!("Part two example cycles: {} (expected 6)", part_two_example_cycles);

    let input_parallel_cycles = run_parallel(&input_graph, input_instructions);
    println!("Part two input cycles: {}", input_parallel_cycles);

}