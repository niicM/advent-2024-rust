use std::marker::PhantomData;
use std::{collections::HashSet, hash::Hash};

use crate::backtrack;

trait Backtrack {
    type State: Hash + Eq;
    type Accum;
    type World;

    fn get_world() -> Self::World;
    fn get_initial(world: &Self::World) -> (Self::State, Self::Accum);
    fn get_next_states(world: &Self::World, state: &Self::State) -> Vec<Self::State>;
    fn get_next_accum(
        world: &Self::World,
        last_accum: &Self::Accum,
        last_state: &Self::State,
        next_state: &Self::State,
    ) -> Self::Accum;
    fn get_score(world: &Self::World, state: &Self::State, accum: &Self::Accum) -> Option<i32>;
}

struct Solver<B: Backtrack> {
    backtrack: PhantomData<B>,
}

impl<B: Backtrack> Solver<B> {
    fn solve(&self) -> (Vec<B::State>, i32) {
        struct Moment<State, Accum> {
            state_index: usize,
            state_list: Vec<State>,
            accum_list: Vec<Accum>,
        }

        let mut best_solution: (Vec<B::State>, i32) = (Vec::new(), std::i32::MIN);
        let mut current_path: Vec<Moment<B::State, B::Accum>> = Vec::new();
        let visited: HashSet<B::State> = HashSet::new();
        let world = B::get_world();
        let (state, accum) = B::get_initial(&world);
        current_path.push(Moment {
            state_index: 0,
            state_list: vec![state],
            accum_list: vec![accum],
        });

        loop {
            let last_moment = current_path.last().unwrap();
            let last_moment_idx = last_moment.state_index;

            if last_moment.state_index < last_moment.state_list.len() {
                // Create and add next moment, keep going forwards

                let next_states: Vec<B::State> =
                    B::get_next_states(&world, &last_moment.state_list[last_moment_idx])
                        .into_iter()
                        .filter(|s| !visited.contains(s))
                        .collect();

                let next_accums: Vec<B::Accum> = next_states
                    .iter()
                    .map(|next_state| {
                        B::get_next_accum(
                            &world,
                            &last_moment.accum_list[last_moment_idx],
                            &last_moment.state_list[last_moment_idx],
                            &next_state,
                        )
                    })
                    .collect();
                
                current_path.push(Moment {
                    state_index: 0,
                    state_list: next_states,
                    accum_list: next_accums,
                });

                let score = B::get_score(&world, &)
            }
        }

        // let accum = B::get_next_accum(&world, &last_accum, &last_state, &next_state);
        // Moment{state_index: 0, state_list: next_states,};
        todo!()
    }
}

// impl Solver {
//     fn solve()
// } 

fn solve<B: Backtrack>(world: B::World) -> (Vec<B::State>, i32) {
    struct Moment<State, Accum> {
        state_index: usize,
        state_list: Vec<State>,
        accum_list: Vec<Accum>,
    }

    let mut best_solution: (Vec<B::State>, i32) = (Vec::new(), std::i32::MIN);
    let mut current_path: Vec<Moment<B::State, B::Accum>> = Vec::new();
    let visited: HashSet<B::State> = HashSet::new();
    let world = B::get_world();
    let (state, accum) = B::get_initial(&world);
    current_path.push(Moment {
        state_index: 0,
        state_list: vec![state],
        accum_list: vec![accum],
    });

    loop {
        let last_moment = current_path.last().unwrap();
        let last_moment_idx = last_moment.state_index;

        if last_moment.state_index < last_moment.state_list.len() {
            // Create and add next moment, keep going forwards

            let next_states: Vec<B::State> =
                B::get_next_states(&world, &last_moment.state_list[last_moment_idx])
                    .into_iter()
                    .filter(|s| !visited.contains(s))
                    .collect();

            let next_accums: Vec<B::Accum> = next_states
                .iter()
                .map(|next_state| {
                    B::get_next_accum(
                        &world,
                        &last_moment.accum_list[last_moment_idx],
                        &last_moment.state_list[last_moment_idx],
                        &next_state,
                    )
                })
                .collect();
            
            current_path.push(Moment {
                state_index: 0,
                state_list: next_states,
                accum_list: next_accums,
            });

            let score = B::get_score(&world, &)
        }
    }

    // let accum = B::get_next_accum(&world, &last_accum, &last_state, &next_state);
    // Moment{state_index: 0, state_list: next_states,};
    todo!()
}