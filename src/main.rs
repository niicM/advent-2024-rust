use std::fmt::{Debug, Display};

mod backtrack;

#[derive(PartialEq, Eq, Clone, Hash)]

struct SolState {
    i: i32,
    j: i32,
}
impl Display for SolState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.i, self.j)
    }
}

impl Debug for SolState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.i, self.j)
    }
}

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct ExtraState {
    direction: Direction,
    cost: i32,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
    let adjacent = vec![
        SolState {
            i: state.i,
            j: state.j + 1,
        },
        SolState {
            i: state.i,
            j: state.j - 1,
        },
        SolState {
            i: state.i + 1,
            j: state.j,
        },
        SolState {
            i: state.i - 1,
            j: state.j,
        },
    ];
    adjacent
        .into_iter()
        .filter(|state| world[state.i as usize][state.j as usize] != MazeTile::Wall)
        .collect()
}


fn main() {
    let file_name = "input_16_d1_7036.txt";
    // let file_name = "input_16.txt";
    let contents: String = std::fs::read_to_string(file_name).unwrap();
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

    // println!("{:?}\n", contents);
    let res = backtrack::solve::<MazeBacktrack>(contents);
    println!("result: {:?}", res);
    println!("len: {:?}", res.0.len());
}

struct MazeBacktrack;

impl backtrack::Backtrack for MazeBacktrack {
    type State = SolState;
    type Accum = ExtraState;
    type World = Vec<Vec<MazeTile>>;

    fn get_initial(world: &Self::World) -> (Self::State, Self::Accum) {
        (
            find_start(world).expect("No starting point in maze!"),
            ExtraState {
                direction: Direction::Right,
                cost: 0,
            },
        )
    }

    fn get_next_states(world: &Self::World, state: &Self::State) -> Vec<Self::State> {
        next_states(state, world)
    }

    fn get_next_accum(
        _world: &Self::World,
        last_accum: &Self::Accum,
        last_state: &Self::State,
        next_state: &Self::State,
    ) -> Self::Accum {
        let direction = if last_state.i == next_state.i {
            if last_state.j < next_state.j {
                Direction::Right
            } else {
                Direction::Left
            }
        } else {
            if last_state.i < next_state.i {
                Direction::Down
            } else {
                Direction::Up
            }
        };
        let cost_increment = if last_accum.direction == direction {
            1
        } else {
            1001
        };

        Self::Accum {
            direction,
            cost: cost_increment + last_accum.cost,
        }
    }

    fn get_score(world: &Self::World, state: &Self::State, accum: &Self::Accum) -> Option<i32> {
        if world[state.i as usize][state.j as usize] == MazeTile::End {
            Some(-accum.cost)
        } else {
            None
        }
    }
}
