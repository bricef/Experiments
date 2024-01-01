
use advent_of_code_2023::libaoc::{read_lines, TextGrid};
use regex::Regex;
use std::{fmt::Debug, rc::Rc};

fn scan_line_for_symbol(lines: &Vec<String>, line_index: usize, start: usize, end: usize) -> bool {
    if let Some(line) = &lines.get(line_index){
        let start_index = if start > 0 { start-1 } else { start };
        let end_index = if end+1 > line.len()-1 {line.len()-1} else {end+1} ;
        let segment = &line[start_index..end_index];
        // println!("Scanning '{}' for symbols...", segment);
        let re = Regex::new("([^.[0-9]])").unwrap();
        return re.is_match(segment)
    } else {
        return false
    }
}

fn scan_for_symbols(
    lines: &Vec<String>, 
    line_index: usize, 
    start: usize, end: usize) -> bool {
    return if line_index > 0 { scan_line_for_symbol(lines, line_index-1, start, end) } else { false }
        || scan_line_for_symbol(lines, line_index, start, end)
        || scan_line_for_symbol(lines, line_index+1, start, end);
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct SerialNumber {
    number: u32
}

impl Debug for SerialNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|", &self.number)
    }
}

fn scan_for_serial_numbers(lines: &Vec<String>) -> Vec<SerialNumber> {
    let mut xs : Vec<SerialNumber> = vec![];
    let re = Regex::new("([0-9]+)").unwrap();
    for (index, line) in lines.iter().enumerate(){
        // println!("{}: {}", index, line);
        for cap in re.captures_iter(line.as_str()){
            if let Some(mat) = cap.get(0) {
                // println!("{:?}", mat);
                if scan_for_symbols(&lines, index, mat.start(), mat.end()){
                    // println!("{} IS PART", mat.as_str());
                    xs.push(SerialNumber{
                        number: mat.as_str().parse().unwrap(),
                        // line: index,
                        // start: mat.start(),
                        // end: mat.end()
                        
                    });
                } else {
                    // println!("{} IS NOT PART", mat.as_str());
                }
            }
        }
        // println!("\n");
    }
    return xs;
}

#[derive(Debug)]
struct Gear {
    first : u32,
    second: u32
}

impl Gear {
    fn ratio(&self) -> u32 {
        return self.first * self.second;
    }
}

fn scan_for_pivots(lines: &Vec<String>) -> Vec<(usize, usize)> {
    let mut pivots : Vec<(usize, usize)> = Vec::new();
    for (lindex, line) in lines.iter().enumerate(){
        for (cindex, c) in line.chars().enumerate(){
            if c == '*' {
                pivots.push((lindex, cindex));
            }
        }
    }
    return pivots;
}

fn scan_and_augment_grid(grid: &mut TextGrid<SerialNumber>) /* -> Vec<SerialNumber> */{
    // let mut sns = vec![];
    let re = Regex::new("([0-9]+)").unwrap();
    let basis = &grid.basis.to_owned();

    for (index, line) in basis.iter().enumerate(){
        // println!("{}: {}", index, line);
        for cap in re.captures_iter(line.as_str()){
            if let Some(mat) = cap.get(0) {
                // println!("{:?}", mat);
                let has_symbols = scan_for_symbols(&basis, index, mat.start(), mat.end());
                if has_symbols {
                    // println!("{} IS PART", mat.as_str());
                    let sn = Rc::new(SerialNumber{ number: mat.as_str().parse().unwrap()});
                    for ci in mat.start()..mat.end() {
                        grid.set_entity(index, ci, sn.clone())
                    }
                    // sns.push(sn);
                } else {
                    // println!("{} IS NOT PART", mat.as_str());
                }
            }
        }
        // println!("\n");
    }
    // return sns;
}


fn scan_for_gears(grid: &TextGrid<SerialNumber>) -> Vec<Gear>{ 
    let mut gears : Vec<Gear> = vec![];    
    
    for pivot in scan_for_pivots(&grid.basis).iter(){
        let entities  = grid.get_entities((pivot.0-1, pivot.1-1), (pivot.0+1, pivot.1+1));
        // println!("ENTITIES FOUND AROUND {:?}: {:?}", pivot, entities);
        if entities.len() >= 2 {
            gears.push(Gear { first: entities[0].number, second: entities[1].number })
        }
    }

    return gears;
}


fn main(){ 
    println!("# Day 03");

    let example_input = read_lines("files/03-example.txt");
    let input = read_lines("files/03-input.txt");
    
    println!("## Part 1");
    let expected_output_parts = 4361;
    let example_numbers = scan_for_serial_numbers(&example_input);
    let example_total: u32 = example_numbers.iter().map(|sn| sn.number ).sum();
    println!("Example Total:{} (should be {})", example_total, expected_output_parts);
    
    let numbers = scan_for_serial_numbers(&input);
    let total : u32 = numbers.iter().map(|sn| sn.number ).sum();
    println!("Input Total:{}", total);

    println!("## Part 2");
    
    let mut example_grid: TextGrid<SerialNumber> = TextGrid::from_file("files/03-example.txt");
    let mut input_grid = TextGrid::from_file("files/03-input.txt");
    scan_and_augment_grid(&mut example_grid);
    scan_and_augment_grid(&mut input_grid);
    // println!("{:?}", grid);
    // println!("@(1,3): {:?}", grid.cells[1][3]);

    let expected_output_gears = 467835;
    let example_gears = scan_for_gears(&example_grid);
    let example_total_ratios : u32 = example_gears.iter().map(|g| g.ratio()).sum();
    println!("Example sum of ratios: {} (should be {})", example_total_ratios, expected_output_gears);

    let input_gears = scan_for_gears(&input_grid);
    let input_total_ratios : u32 = input_gears.iter().map(|g| g.ratio()).sum();
    println!("Input total Gears: {}", input_total_ratios);






}
