
use regex::Regex;
use aho_corasick::AhoCorasick;
use advent_of_code_2023::libaoc::read_lines;


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }


fn line_to_digits(line:&String) -> Vec<u32>{
    let ds: Vec<u32> = line.chars()
        .filter(|c| c.is_ascii_digit() )
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    return ds
}

fn parse_value(s: &str) -> u32 {
    // println!("v: {}",s);
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<u32>().unwrap()
    }
}

#[allow(dead_code)]
fn line_to_digits_including_words_regex(line: &String) -> Vec<u32> {
    let patterns= "[0-9]|one|two|three|four|five|six|seven|eight|nine";
    let re = Regex::new(patterns).unwrap();
    let tokens = re.captures_iter(line);
    let digits :Vec<u32>= tokens
        .map(|c| c.extract::<0>() )
        .map(|(s, _)|s)
        .map(parse_value)
        .collect();
    return digits;    
}

fn line_to_digits_including_words_aho_corasick(line: &String) -> Vec<u32> {
    let patterns= [
        "1","2","3","4","5","6","7","8","9",
        "one","two","three","four","five","six","seven","eight","nine"];
    let re = AhoCorasick::new(patterns).unwrap();
    let digits: Vec<u32> = re
        .find_overlapping_iter(line)
        .map(|m| &line[m.start()..m.end()])
        .map(parse_value)
        .collect();
    return digits;    
}


fn concatenate_of_first_and_last_digits(ds: Vec<u32>) -> u32 {
    if let Some(first) = ds.first() {
        let v: u32;
        if let Some(x) =  ds.last(){
            v = 10*first + x;
        }else{
            v = 10*first+  first;
        }
        return v;
    }
    return 0;
    
}

#[allow(dead_code)]
fn debug_output(f: fn(&String)->Vec<u32>) -> impl Fn(&String) -> Vec<u32>{
    let inf = move |s:&String| {
        println!("input: {}", s);
        
        let r =  f(s);
        println!("digits: {}", r.iter().map(|x| x.to_string()).collect::<String>() );
        return r;
    };
    return inf;
}

#[allow(dead_code)]
fn printer(t: u32) -> u32 {
    println!("{}", t);
    return t;
}

fn main() {
    println!("Advent of code day 01");
    let lines = read_lines("files/01-input.txt");
    
    let total_simple = lines.iter()
        .map(line_to_digits)
        .map(concatenate_of_first_and_last_digits)
        .reduce(|acc, e| acc+e).unwrap();
    println!("digits only: {}", total_simple);

    let total_complex: u32 = lines.iter()
        .map(line_to_digits_including_words_aho_corasick)
            .map(concatenate_of_first_and_last_digits)
            // .map(printer)
            .reduce(|acc, e| acc+e).unwrap();
    println!("digits and words: {}", total_complex);

}