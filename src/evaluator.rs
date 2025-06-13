//! This file is where you will implement your code.

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use crate::{Fsm, VendingAction, DENOMINATIONS, MAX_CENTS};

pub fn price_branch(
    price_map: &HashMap<Arc<str>, u32>,
    mut trans_lst: HashMap<(u32, VendingAction), u32>,
    mut dist: HashMap<u32, u32>,
    mut count: u32,
) -> HashMap<(u32, VendingAction), u32> {
    let mut queue: VecDeque<u32> = VecDeque::new();
    for n in 0..count + 1 {
        queue.push_back(n);
    }
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        for price in price_map {
            if *dist.get(&curr).unwrap() >= *price.1 {
                // if distance > current price
                let new_dist = *dist.get(&curr).unwrap() - *price.1;
                let search = dist
                    .iter()
                    .find_map(|(k, v)| if *v == new_dist { Some(k) } else { None });
                if search.is_some() {
                    // search.is_some() implies search != None
                    trans_lst.insert(
                        (curr, VendingAction::Vend(price.0.clone())),
                        *search.unwrap(),
                    );
                    // println!("Added ({}, vend {}) => {}, total cost is {}",curr,price.0.clone(),*search.unwrap(),new_dist);
                } else {
                    count += 1;
                    trans_lst.insert((curr, VendingAction::Vend(price.0.clone())), count);
                    dist.insert(count, new_dist);
                    // println!("Added ({}, vend {}) => {}, total cost is {}",curr,price.0.clone(),count,new_dist);
                    queue.push_back(count);
                }
            }
        }
    }
    return trans_lst;
}

impl Fsm {
    pub fn create(price_map: &HashMap<Arc<str>, u32>) -> Fsm {
        let mut trans_lst: HashMap<(u32, VendingAction), u32> = HashMap::new();
        let mut dist: HashMap<u32, u32> = HashMap::from([(0, 0)]);
        let mut queue: VecDeque<u32> = VecDeque::from([0]);
        let mut count: u32 = 0;
        while !queue.is_empty() {
            let curr = queue.pop_front().unwrap();
            for coin in DENOMINATIONS {
                let total_cost = dist.get(&curr).unwrap() + coin;
                if total_cost <= MAX_CENTS {
                    // Check for node with same distance first
                    let search =
                        dist.iter()
                            .find_map(|(k, v)| if *v == total_cost { Some(k) } else { None });
                    if search.is_some() {
                        // search.is_some() = true implies search != None
                        trans_lst.insert((curr, VendingAction::Insert(coin)), *search.unwrap());
                        // println!("Added ({}, insert {}) => {}, total cost is {}",curr,coin,*search.unwrap(),total_cost);
                    } else {
                        count += 1;
                        trans_lst.insert((curr, VendingAction::Insert(coin)), count);
                        dist.insert(count, total_cost);
                        // println!("Added ({}, insert {}) => {}, total cost is {}",curr,coin,count,total_cost);
                        queue.push_back(count);
                    }
                }
            }
        }
        trans_lst = price_branch(price_map, trans_lst, dist, count);
        return Fsm {
            start: 0,
            transitions: trans_lst,
        };
    }

    /// Validate the given list of actions on the FSM.
    pub fn eval(&self, actions: &[VendingAction]) -> bool {
        let mut queue: VecDeque<VendingAction> = VecDeque::new();
        for action in actions {
            queue.push_back(action.clone());
        }
        let mut curr_node = self.start;
        while !queue.is_empty() {
            // The queue only holds the actions and will not be updated
            let curr_action = queue.pop_front().unwrap();
            let search = self.transitions.iter().find_map(|(k, v)| {
                if k.0 == curr_node && k.1 == curr_action {
                    Some(v)
                } else {
                    None
                }
            });
            if search.is_some() {
                // search.is_some() = true implies search != None
                // Current node is updated with the resulting node
                curr_node = *search.unwrap();
            } else {
                // Returns false if the current node can't take the current action
                return false;
            }
        }
        // Returns true when all actions have been dequeued
        return true;
    }
}
