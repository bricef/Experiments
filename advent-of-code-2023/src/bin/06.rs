
use itertools::Itertools;


struct Race {
    duration: u64, 
    best: u64
}

impl Race {
    const fn from(d: u64, b: u64) -> Race {
        return Race{
            duration: d,
            best: b
        }
    }
}

const EXAMPLE_RACES: [Race; 3]  = [
    Race::from(7, 9),
    Race::from(15, 40),
    Race::from(30, 200)
];

const INPUT_RACES: [Race; 4] = [
    Race::from(35,212),
    Race::from(93,2060),
    Race::from(73,1201),
    Race::from(66,1044)
];

#[allow(dead_code)]
#[derive(Debug)]
struct RaceSolution {
    accelerate_time: u64,
    total_distance:u64
}

fn distance(speed:u64 , time:u64)->u64{
    return speed * time
}

fn gen_solutions(race_time: u64) -> Vec<RaceSolution>{
    let mut sols : Vec<RaceSolution> = vec![];
    for acc_time in 0..race_time{
        sols.push(RaceSolution {
            accelerate_time: acc_time,
            total_distance: distance(acc_time, race_time-acc_time)
        })
    }
    return sols;
}

fn race_to_good_sols(r: &Race) -> Vec<RaceSolution> {
    let sols = gen_solutions(r.duration);
    let good_sols = sols.into_iter()
        .filter(|sol| sol.total_distance > r.best)
        .collect_vec();
    return good_sols;
}


fn main(){
    let example_total:usize = EXAMPLE_RACES.iter().map(|r| race_to_good_sols(r).len()).product();
    let input_total: usize = INPUT_RACES.iter().map(|r| race_to_good_sols(r).len()).product();

    println!("# Day 06");
    println!("## Part 1");
    println!("EXAMPLE TOTAL: {:?} (Expected 288)", example_total);
    println!("INPUT TOTAL: {:?}", input_total);
    
    println!("## Part 2");
    let example_race = Race{
        duration:71530,
        best: 940200
    }; 
    let input_race = Race{
        duration: 35937366,
        best: 212206012011044
    };
    println!("Can win example race {} times (expected 71503)", race_to_good_sols(&example_race).len());
    println!("Can win input race {} times", race_to_good_sols(&input_race).len());
}