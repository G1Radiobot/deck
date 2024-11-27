use core::fmt;
use std::fmt::Display;
use rand::*;

pub trait Card {
    fn populate(many: u8) -> Vec<Box<Self>>;
}

pub struct StandardCard<'a> {
    suite: &'a str,
    value: u8
}

impl<'a> Card for StandardCard<'a> {
    fn populate(many: u8) -> Vec<Box<Self>> {
        let suite_list = vec!["Spades", "Hearts", "Clubs", "Diamonds"];
        let mut rtn = Vec::<Box<StandardCard>>::with_capacity(52);
        for _ in 0..many {
            for i in 0..13 {
                for suite in suite_list.iter() {
                    rtn.push(Box::new(StandardCard {
                        suite: suite,
                        value: (|val: u8| -> u8 {
                            val + 1
                        })(i)
                    }))
                }
            }
        }
        rtn
    }
}


pub struct Deck<C>(Vec<C>);

impl<C> Deck<C> {
    pub fn new() -> Deck<C>{
        Deck(Vec::<C>::new())
    }

    pub fn new_from_vec(vector: Vec<Box<C>>) -> Self {
        let mut new_vec = Vec::<C>::with_capacity(52);
        for i in vector.into_iter() {
            new_vec.push(*i);
        }
        Deck(new_vec)
    }

    pub fn shuffle(&mut self) {
        for i in (1..self.0.len()).rev() {
            self.0.swap(thread_rng().gen_range(0..i), i);
        }
    }

    pub fn deal(&mut self, other: &mut Self, many: u8) -> bool {
        let mut i = 0;
        loop {
            if let Some(new_card) = self.0.pop() {
                other.0.push(new_card);
            }
            else {break false}
            i = i + 1;
            if i >= many {
                break true
            }
        }
    }

    
    pub fn peak(&self, index: usize) -> &C {
        &self.0[index]
    }

    pub fn peak_hand(&self) -> &Vec<C> {
        &self.0
    }
}

impl<'a> Display for StandardCard<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            1 => write!(f, "Ace of {}", self.suite),
            11 => write!(f, "Jack of {}", self.suite),
            12 => write!(f, "Queen of {}", self.suite),
            13 => write!(f, "King of {}", self.suite),
            _ => write!(f, "{} of {}", self.value.to_string(), self.suite)
        }
    }
}

#[cfg(test)]
mod test {
use crate::Card;

use super::Deck;
use super::StandardCard;
   
    #[test]
    fn populate_test() {
        let mut bob = Deck::new_from_vec(StandardCard::populate(1));
        let mut steve = Deck::<StandardCard>::new();
        bob.shuffle();
        bob.deal(&mut steve, 5);

        for i in &steve.0 {
            println!("{}", i)
        }
    }
}