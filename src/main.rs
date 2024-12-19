use std::collections::HashSet;

mod backtrack;

#[derive(Hash)]
struct SolState {
    i: i32,
    j: i32,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct ExtraState {
    direction: Direction,
}

#[derive(Debug, PartialEq)]
enum MazeTile {
    Wall,
    Corridor,
    Start,
    End,
}

impl MazeTile {
    fn from_char(ch: char) -> Self {
        match ch {
            '#' => MazeTile::Wall,
            '.' => MazeTile::Corridor,
            'S' => MazeTile::Start,
            'E' => MazeTile::End,
            _ => panic!("Invalid Input!"),
        }
    }
}

fn find_start(world: &Vec<Vec<MazeTile>>) -> Option<SolState> {
    for i in 0..world.len() {
        for j in 0..world[0].len() {
            if world[i][j] == MazeTile::Start {
                return Some(SolState {
                    i: i as i32,
                    j: j as i32,
                });
            }
        }
    }
    None
}

fn next_states(state: &SolState, world: &Vec<Vec<MazeTile>>) -> Vec<SolState> {
    let mut adjacent = Vec::new();
    for i in -1..=1 {
        for j in -1..=1 {
            if i != 0 && j != 0 {
                let tile_i = state.i + i;
                let tile_j = state.j + j;
                let tile = &world[tile_i as usize][tile_j as usize];
                if *tile != MazeTile::Wall {
                    adjacent.push(SolState {
                        i: tile_i,
                        j: tile_j,
                    });
                }
            }
        }
    }
    adjacent
}

fn main() {
    let contents: String = std::fs::read_to_string("input_16.txt").unwrap();
    let contents: Vec<&str> = contents.split("\n").collect();
    let contents: Vec<Vec<MazeTile>> = contents
        .iter()
        .map(|row| {
            (*row)
                .chars()
                .map(MazeTile::from_char)
                .collect::<Vec<MazeTile>>()
        })
        .collect();

    println!("{:?}\n", contents);
}
