use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, mut min: u32, mut max: u32) -> u32 {
        let mid = (min + max) / 2; 
        if min >= max {
            return min;
        }
        match player.ask_to_compare(mid) {
            0 => return mid,
            1 => { return Self::guess_the_number(player, mid+1, max) },
            -1 => { return Self::guess_the_number(player, min, mid-1) },
            _ => unreachable!(),
            }
        }
}


    