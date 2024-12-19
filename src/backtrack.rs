use std::{collections::HashSet, hash::Hash};

trait Backtrack {
    type State: Hash + Eq + Clone;
    type Accum: Clone;
    type World;

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

fn solve<B: Backtrack>(world: B::World) -> (Vec<B::State>, i32) {
    #[derive(Clone)]
    struct Moment<State, Accum> {
        state_index: usize,
        state_list: Vec<State>,
        accum_list: Vec<Accum>,
    }

    let mut best_solution: (Vec<B::State>, i32) = (Vec::new(), std::i32::MIN);
    let mut current_path: Vec<Moment<B::State, B::Accum>> = Vec::new();
    let visited: HashSet<B::State> = HashSet::new();

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
        } else {
            // backtrack

            while let Some(last_moment) = current_path.last_mut() {
                let depleted_choices = last_moment.state_index >= last_moment.state_list.len() - 1;
                if depleted_choices {
                    current_path.pop();
                    continue;
                } else {
                    last_moment.state_index += 1;
                    break;
                }
            }
        }

        if current_path.is_empty() {
            break;
        }
        // Check the score of the new state

        let last_moment = current_path.last().unwrap();
        let last_moment_idx = last_moment.state_index;

        let score = B::get_score(
            &world,
            &last_moment.state_list[last_moment_idx],
            &last_moment.accum_list[last_moment_idx],
        );

        if let Some(score) = score {
            if score > best_solution.1 {
                let path = current_path
                    .iter()
                    .map(|moment| moment.state_list[moment.state_index].clone())
                    .collect();
                best_solution = (path, score);
            }
        }
    }

    best_solution
}
