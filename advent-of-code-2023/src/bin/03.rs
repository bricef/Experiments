
use advent_of_code_2023::libaoc::read_lines;
use regex::{Regex, Match};

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

struct SerialNumber {
    number: u32,
    line: usize,
    start: usize,
    end: usize,
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
                        line: index,
                        start: mat.start(),
                        end: mat.end()
                        
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

    // println!("## Part 2");
    // let expected_output_gears = 467835;
    // let gears = scan_for_gears(&example_input);
    // let example_total_ratios : u32 = gears.iter().map(|g| g.ratio()).sum();
    // println!("Example sum of ratios: {} (should be {})", example_total_ratios, expected_output_gears);






}
