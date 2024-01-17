
use std::{fs::read_to_string, fmt, rc::Rc};
use core::fmt::Debug;
use std::cmp::Eq;
use std::hash::Hash;
use itertools::Itertools;

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


pub struct TextGrid<T> {
    pub basis: Vec<String>,
    pub cells: Vec<Vec<GridCell<T>>>,
}

#[derive(Clone)]
pub struct GridCell<T>{
    pub line: usize,
    pub offset: usize,
    pub value: char,
    pub entity: Option<Rc<T>>
}

impl<T> fmt::Debug for GridCell<T> where T: Debug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("GridCell").field("line", &self.line).field("offset", &self.offset).field("value", &self.value).field("entity", &self.entity).finish()
        write!(f, "({},{})[{}]: {:?}", &self.line, &self.offset, &self.value, &self.entity)
    }
}

fn cells_from_lines<T>(lines: &Vec<String>) -> Vec<Vec<GridCell<T>>>{
    let r = lines.iter().enumerate().map(|(line_index, line)| 
            line.chars().enumerate().map(|(char_index, c)| {
                let gc:GridCell<T> = GridCell::<T> {
                    line: line_index,
                    offset: char_index,
                    value: c,
                    entity: None
                };
                return gc;
            }
            ).collect()
        ).collect();
    return r;
}

impl<T> fmt::Debug for TextGrid<T> where T: Clone + Debug{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextGrid").field("basis", &self.basis).field("cells", &self.cells).finish()
    }
}

impl<'a, T> TextGrid <T>
    where T : Clone + Debug + Eq + Hash
{
    pub fn from_file(filepath: &str) -> TextGrid<T> {
       let basis = read_lines(&filepath);
        TextGrid{
            cells: cells_from_lines(&basis),
            basis: basis
        }
    }
    pub fn from_lines(lines: &Vec<String>) -> TextGrid<T> {
        TextGrid{
            basis: lines.to_vec(),
            cells: cells_from_lines(lines)
        }
    }
    pub fn get_entity(&self, line: usize, offset:usize) -> Option<Rc<T>>{
        if line > self.cells.len()-1 { return None }
        if offset > self.cells[line].len()-1 { return None }

        let row = &self.cells[line];
        let cell = &row[offset];
        return cell.entity.to_owned();
    }

    pub fn set_entity(&mut self, line: usize, offset:usize, entity: Rc<T>){
        if line > self.cells.len()-1 { panic!("Out of bounds set to TextGrid. After EOF.") }
        if offset > self.cells[line].len()-1 { panic!("Out of bounds set to TextGrid. After EOL.") }

        let gc = &mut self.cells[line][offset];
        gc.entity = Some(entity);
    }

    pub fn scan<X>(&self, scanner: X)
        where X: Fn(&TextGrid<T>, &GridCell<T>) -> Option<T>
    {   
        for row in &self.cells {
            for cell in row {
                let gc = cell;
                let _entity = scanner(self, &gc);
            }
        }
    }

    pub fn get_cell(&self, coord: (usize, usize)) -> Option<&GridCell<T>>{
        let (line,offset) = coord;
        if line > self.cells.len()-1 { return None }
        if offset > self.cells[line].len()-1 { return None }
        Some(&self.cells[line][offset])
    }

    pub fn get_cells(&self, start:(usize, usize), end: (usize,usize)) -> Vec<&GridCell<T>> {
        let mut gcs = vec![];
        for d in start.0..end.0+1 {
            for o in start.1..end.1+1 {
                // println!("Looking up ({},{})", d, o);
                gcs.push(&self.cells[d][o])
            }
        }
        return gcs; 
    }

    pub fn get_entities(&self, start:(usize, usize), end: (usize,usize)) -> Vec<Rc<T>>{
        // println!("get_entities from {:?} to {:?}", start, end);
        let cells = self.get_cells(start, end);
        let mut rcs = vec![];
        for cell in cells {
            if let Some(rc) = &cell.entity{
                rcs.push(rc.to_owned())
            }
        }
        return rcs.into_iter().unique().collect();
    }

}