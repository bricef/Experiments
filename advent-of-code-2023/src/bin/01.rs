use std::fs::read_to_string;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

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

fn line_to_digits_including_words(line: &String) -> Vec<u32> {
    println!("{}", line);
    let re = Regex::new("([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let tokens = re.captures_iter(line);
    let digits :Vec<u32>= tokens
        .map(|c| c.extract::<1>() )
        .map(|(s,_)|s)
        .map(parse_value)
        .collect();
    println!("{}", digits.iter().map(|x| x.to_string()).collect::<String>() );
    return digits;    
}

fn concatenate_of_first_and_last_digits(ds: Vec<u32>) -> u32 {
    let first = ds.first().unwrap();
    let v: u32;
    if let Some(x) =  ds.last(){
        v = 10*first + x;
    }else{
        v = 10*first+  first;
    }
    println!("{}",v);
    return v;
}

fn main() {
    println!("Advent of code day 01");
    let lines = read_lines("files/01-example-2.txt");
    
    let total_simple = lines.iter().map(line_to_digits).map(concatenate_of_first_and_last_digits).reduce(|acc, e| acc+e).unwrap();
    println!("digits only: {}", total_simple);

    let total_complex: u32 = lines.iter().map(line_to_digits_including_words).map(concatenate_of_first_and_last_digits).reduce(|acc, e| acc+e).unwrap();
    println!("digits and words: {}", total_complex);

}