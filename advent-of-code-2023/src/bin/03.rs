
use advent_of_code_2023::libaoc::read_lines;
use regex::Regex;





fn get_at(lines: Vec<String>, line_index: usize, offset:usize) -> Option<char>{
    if line_index >= lines.len() { return None }
    let line = lines.get(line_index);
    if offset >= line?.len() {return None }
    return line?.chars().nth(offset);
}

fn scan_for_symbols(
    lines: Vec<String>, 
    line_index: usize, 
    start: u32, end: u32) -> bool {
    
    lines.get(line_index);
    return false
}

fn scan_for_numbers(lines: Vec<String>) -> Vec<u32> {
    let re = Regex::new("([0-9]+)").unwrap();
    for (index, line) in lines.iter().enumerate(){
        println!("{}: {}", index, line);
        for cap in re.captures_iter(line.as_str()){
            let mat = cap.get(0);
            println!("{:?}", mat);
        }
        println!("\n");
    }
    return Vec::from([1,2,3]);
}




fn main(){ 
    let example_input = read_lines("files/03-example.txt");
    let _example_output = 4361;
    scan_for_numbers(example_input);


}
