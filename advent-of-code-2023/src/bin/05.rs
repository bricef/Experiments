use std::{fs::read_to_string, collections::HashMap, u64::MAX};

#[derive(Debug, Clone)]
struct Mapping {
    out_start: u64,
    in_start: u64,
    range: u64,
}

#[derive(Debug)]
struct GardenMap{
    pub input: String,
    pub output: String,
    mappings: Vec<Mapping>
} 

impl Clone for GardenMap {
    fn clone(&self) -> Self {
        Self { 
            input: self.input.clone(), 
            output: self.output.clone(), 
            mappings: self.mappings.clone() 
        }
    }
}


impl GardenMap {
    fn from_string_specifier(s: &str) -> GardenMap {
        let lines = s.split("\n").collect::<Vec<&str>>();
        let title = lines[0].split_whitespace().collect::<Vec<&str>>()[0].split("-to-").collect::<Vec<&str>>();
        let mappings = lines[1..].iter()
            .map(|l| {
                let ns = l.split_whitespace().map(|ns| ns.parse::<u64>().unwrap()).collect::<Vec<u64>>();
                Mapping{
                    out_start: ns[0],
                    in_start: ns[1],
                    range: ns[2],
                }
            }).collect::<Vec<Mapping>>();
        GardenMap {
            input: String::from(title[0]),
            output: String::from(title[1]),
            mappings: mappings
        }
    }
    fn map(&self, i: u64) -> u64 {
        // println!("-> {}", i);
        for m in &self.mappings {
            if i >= m.in_start && i < m.in_start+m.range {
                return m.out_start + (i-m.in_start)
            }
        }
        return i
    }
}

fn map_x_to_y(metamap:&HashMap<String, GardenMap>, input: String, output: String, value: u64) -> Option<u64> {
    let mut i= value;
    let mut current = input;

    loop {
        if let Some(m) = metamap.get(&current) {
            i = m.map(i);
            current = String::from(&m.output);
            if current == output { return Some(i); }
        } else {
            return None;
        }   
    }
}

fn map_seed_to_location(metamap:&HashMap<String, GardenMap>, seed: u64) -> Option<u64> {
    map_x_to_y(metamap,String::from("seed"), String::from("location"), seed)
}

fn locations_from_seeds(metamap:&HashMap<String, GardenMap>, seeds: &Vec<u64>) -> Vec<u64>{
    seeds.iter()
        .map(|seed| map_seed_to_location(&metamap, *seed).unwrap())
        .collect::<Vec<u64>>()
}

fn parse_garden(s:&str) -> (HashMap<String, GardenMap>, Vec<u64>,Vec<u64>){
    let sections : Vec<&str> = s.split("\n\n").collect();
    let seeds = sections[0].split(":").collect::<Vec<&str>>()[1].split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let maps = sections[1..].iter().map(|sec| GardenMap::from_string_specifier(sec)).collect::<Vec<GardenMap>>();
    let metamap: HashMap<String, GardenMap> = HashMap::from_iter(maps.iter().map(|m| (String::from(&m.input), m.clone())));
    let locations = locations_from_seeds(&metamap, &seeds);
    return (metamap, seeds, locations)
}

fn seed_ranges_to_seeds(srs: &Vec<u64>) -> Vec<u64> {
    srs.chunks(2)
        .flat_map(|rdef: &[u64]| {
            // println!("RDEF {:?}", rdef);
            (rdef[0]..rdef[0]+rdef[1]).collect::<Vec<u64>>()
        })
        .collect::<Vec<u64>>()
}


fn main () {

    let example = read_to_string("files/05-example.txt").unwrap();
    let input = read_to_string("files/05-input.txt").unwrap();

    

    println!("# Day 05");

    println!("## Part 1");
    let (example_mm, example_seeds, example_locations) = parse_garden(&example);
    println!("Example Seeds: {:?}", example_seeds);
    println!("Example Locations: {:?}", example_locations);
    println!("Example Lowest Location: {} (expect 35)", example_locations.iter().min().unwrap());

    let (input_mm, input_seeds, input_locations) = parse_garden(&input);
    println!("Input Seeds: {:?}", input_seeds);
    println!("Input Locations: {:?}", input_locations);
    println!("Input Lowest Location: {}", input_locations.iter().min().unwrap());

    println!("## Part 2");
    let example_seeds_ranges = seed_ranges_to_seeds(&example_seeds); 
    let example_range_locations= locations_from_seeds(&example_mm, &example_seeds_ranges);
    // println!("Example seed ranges: {:?} (len {})", example_seeds_ranges, example_seeds_ranges.len());
    // println!("Example locations: {:?}", example_range_locations);
    println!("Example lowest location: {:?} (expected 46)", example_range_locations.iter().min().unwrap());


    /*
     * This is very expensive and unsuitable for purpose. It took ~1.5h to complete
     * and should be refactored to use a range analysis algorithm instead.
     */
    let mut lowest_location = MAX;
    let mut total_to_check: u64= 0;
    for chunk in input_seeds.chunks(2) {
        total_to_check += chunk[1];
    }
    println!("Total to check: {}", total_to_check);
    for chunk in input_seeds.chunks(2) {
        println!("From {} to {}", chunk[0], (chunk[0]+chunk[1]));
        for seed in chunk[0]..(chunk[0]+chunk[1]) {
            let location = map_seed_to_location(&input_mm, seed).unwrap();
            if location < lowest_location {
                lowest_location = location
            }
        }
    }
    // let input_seeds_ranges = seed_ranges_to_seeds(&input_seeds);
    // let input_range_locations = locations_from_seeds(&input_mm, &input_seeds_ranges);
    println!("Input lowest Location: {:?}",lowest_location);

    
}