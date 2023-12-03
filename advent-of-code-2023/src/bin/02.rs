
// game is made of rounds, 
// rounds are made of turns, 
// turns are made of colours - number pairs.

use std::collections::HashMap;

use advent_of_code_2023::libaoc::read_lines;


#[derive(Debug)]
struct Game{
    index: u32,
    turns: Vec<Turn>
}


#[derive(Debug)]
struct Turn{
    cubes: HashMap<String,u32>
}

fn parse_turn(round_str:&&str) -> Turn{
    // 1 blue, 2 green
    let map = round_str.split(",")
        .map(|cube_str:&str| {
            // 1 blue
            let cubedef:Vec<_> = cube_str.split(' ').collect();
            (String::from(cubedef[2]), cubedef[1].parse::<u32>().unwrap())
        });
    Turn{
        cubes: HashMap::from_iter(map)
    }
    
}

fn parse_turns(rounds_str: &str) -> Vec<Turn> {
    // 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    let l:Vec<_> = rounds_str.split(";").collect();
    let turns = l.iter().map(parse_turn).collect();
    return turns
}

fn line_to_game(line:&String) -> Game {
    // >Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    
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

fn turn_possible_given_constraint(constraint: &HashMap<String, u32>, turn: &Turn) -> bool{
    
    let possible: bool = turn.cubes.iter()
        .all(|(color,num)| constraint.get(color) >= Some(num));
    // println!("{:?} is possible? {}", turn, possible);
    return possible;
}

fn game_possible_given_constraint(constraint: &HashMap<String, u32>, game: &&Game) -> bool {
    //for every turn in a game, is the value higher than the constriant?
    let valid_games:bool= game.turns.iter()
        .all(|t| turn_possible_given_constraint(&constraint, t));
        
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

    let games:Vec<_> = lines.iter().map(line_to_game).collect();
    println!("CONSTRAINT: {:?}", constraints);
    // println!("ALL GAMES: {:?}", games);
    let valid_games:Vec<_> = games.iter()
        .filter(|g| game_possible_given_constraint(&constraints, g)).collect();
    // println!("VALID GAMES: {:?}", valid_games);
    let sum_indices = valid_games.iter().fold(0, |acc, g| acc + g.index);
    println!("SUM OF INDICES FOR VALID GAMES: {}", sum_indices);



    return;
}