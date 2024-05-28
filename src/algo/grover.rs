use rand::{distributions::WeightedIndex, prelude::*};

use crate::qbit::structs::*;

pub struct GroverState<const T: usize> {
    good_states: Vec<[bool; T]>,
}

impl<const T: usize> GroverState<T> {
    pub fn new(good_states: Vec<[bool; T]>) -> Self {
        Self { good_states }
    }

    pub fn collapse(&self) -> Vec<bool> {
        let [weight_positive, weight_negative] = [8, 2];
        let mut combination: Vec<Vec<bool>> = vec![vec![false], vec![true]];
        for _ in 1..T {
            let temp = combination.clone();
            combination.clear();
            for val in temp {
                let mut added_value_true = val.clone();
                let mut added_value_false = val.clone();
                added_value_true.push(false);
                added_value_false.push(true);
                combination.push(added_value_false);
                combination.push(added_value_true);
            }
        }
        let mut weights = vec![];
        for comb in &combination {
            if self.good_states.iter().find(|val| **val == **comb).is_some() {
                weights.push(weight_positive);
            } else {
                weights.push(weight_negative);
            }
        }
        let weighted_index = WeightedIndex::new(weights).unwrap();
        let mut rng = thread_rng();

        combination[weighted_index.sample(&mut rng)].clone()
    }
}
