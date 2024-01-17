#![allow(dead_code)]

use advent_of_code_2023::libaoc::TextGrid;

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal;

For visualisation, use $ tr 'LJ7F|-' '┗┛┓┏┃━'
*/

const N_S: char = '|';
const E_W: char = '-';
const N_E: char = 'L';
const N_W: char = 'J';
const S_W: char = '7';
const S_E: char = 'F';

const TR_TABLE: &'static [(char,char)] = &[
    (N_S,'┃'),
    (E_W,'━'),
    (N_E,'┗'),
    (N_W,'┛'),
    (S_W,'┓'),
    (S_E,'┏'),
];

type coord = (usize,usize);

fn start_pos<T>(grid: &TextGrid<T>) -> Option<coord>{
    for (line, row) in grid.cells.iter().enumerate() {
        for (offset, gc) in row.iter().enumerate() {
            if gc.value == 'S' { return Some((line,offset))}
        }
    }
    None
}

fn next(grid: &TextGrid<u64>, previous: coord, current: coord) -> Option<coord> {
    let cell = grid.get_cell(current).unwrap();
    if previous.0 < current.0 { // From above
        match cell.value {
            N_S => return Some((current.0+1, current.1)),
            N_E => return Some((current.0, current.1+1)),
            N_W => return Some((current.0, current.1-1)),
            _ => panic!("Could not proceed from {:?} with previous {:?} given {}", current, previous, cell.value)
        }
    } 
    if previous.0 > current.0 { } // From below
    if previous.1 > current.1 { } // from right
    if previous.1 < current.1 { } // from left
    None
}

fn main () {
    let example_one: TextGrid<u64> = TextGrid::from_file("files/10-example-1.txt");
    let example_two: TextGrid<u64> = TextGrid::from_file("files/10-example-2.txt");
    let input: TextGrid<u64> = TextGrid::from_file("files/10-input.txt");


    
}