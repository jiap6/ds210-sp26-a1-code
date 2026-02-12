use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        while (max > min){
            let mid = (min + max) / 2; 
            match player.ask_to_compare(mid) {
                0 => return mid,
                -1 => { max = mid - 1; },
                1 => { min = mid + 1; },
               _ => unreachable!(),
            }
        }
        min
    }
}


    