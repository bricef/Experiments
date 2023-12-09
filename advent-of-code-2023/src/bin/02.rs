
// game is made of rounds, 
// rounds are made of turns, 
// turns are made of colours - number pairs.

use std::collections::HashMap;
use std::cmp;

use advent_of_code_2023::libaoc::read_lines;


#[derive(Debug)]
struct Game{
    index: u32,
    turns: Vec<Turn>
}


type Turn = HashMap<String,u32>;
type Constraint = HashMap<String,u32>;

fn parse_turn(round_str:&&str) -> Turn{
    // 1 blue, 2 green
    let map = round_str.split(",")
        .map(|cube_str:&str| {
            // 1 blue
            let cubedef:Vec<_> = cube_str.split(' ').collect();
            (String::from(cubedef[2]), cubedef[1].parse::<u32>().unwrap())
        });
    HashMap::from_iter(map)
}

fn parse_turns(rounds_str: &str) -> Vec<Turn> {
    // 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let l:Vec<_> = rounds_str.split(";").collect();
    let turns = l.iter().map(parse_turn).collect();
    return turns
}

fn line_to_game(line:&String) -> Game {
    // >Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    println!("{}", line);
    //Quick and (very) dirty parsing...
    let l: Vec<_>= line.split(":").collect();
    let game_str = l[0];
    let rounds_str = l[1];
    let game_index_str = &game_str[5..game_str.len()];
    
    let game_index = game_index_str.parse::<u32>().unwrap();

    return Game {
        index: game_index,
        turns: parse_turns(rounds_str)
    };
}


fn merge_with_operator<K:std::cmp::Eq+std::hash::Hash+std::clone::Clone,V:std::marker::Copy>(op: impl Fn(V,V)->V, m1: &HashMap<K,V>, m2: &HashMap<K,V> ) -> HashMap<K,V>{
    let mut m: HashMap<K, V> = HashMap::new();
    for key in m1.keys().chain(m2.keys()){
        if let (Some(v1), Some(v2)) = (m1.get(&key), m2.get(&key)) {
            m.insert(key.clone(), op(*v1,*v2));
        }else if let Some(v1) = m1.get(&key){
            m.insert(key.clone(), *v1);
        }else if let Some(v2) = m2.get(&key) {
            m.insert(key.clone(), *v2);
        }
    }
    return m

}

fn max_required(turns: Vec<Turn>) -> Constraint{
    let con: Constraint = turns.iter()
        .fold(HashMap::from([]), |acc, t| 
            merge_with_operator(cmp::max, &acc, t));
    return con
}


fn turn_possible_given_constraint(constraint: &Constraint, turn: &Turn) -> bool{
    let possible: bool = turn.iter()
        .all(|(color,num)| constraint.get(color) >= Some(num));
    // println!("{:?} is possible? {}", turn, possible);
    return possible;
}

fn game_possible_given_constraint(constraint: &HashMap<String, u32>, game: &Game) -> bool {
    //for every turn in a game, is the value higher than the constriant?
    println!("GAME {:?}", game);
    println!("CONSTRAINT {:?}", constraint);
    let valid_games:bool= game.turns.iter()
        .all(|t| turn_possible_given_constraint(&constraint, t));
    println!("GAME VALID? {}\n", valid_games);
        
    return valid_games
}

fn main(){
    println!("## Advent of code day 02");
    
    let lines = read_lines("files/02-input.txt");

    let constraints: HashMap<String, u32, _> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14)
    ]);

    println!("CONSTRAINT: {:?}", constraints);
    let valid_games:Vec<_> = lines.iter()
        .map(line_to_game)
        .filter(|g| game_possible_given_constraint(&constraints, g))
        .collect();
    // println!("VALID GAMES: {:?}", valid_games);
    let sum_indices = valid_games.iter().fold(0, |acc, g| acc + g.index);
    println!("SUM OF INDICES FOR VALID GAMES: {}", sum_indices);



    return;
}