use core::panic;

use crate::utils::{self, Solution};

#[derive(Default)]
pub struct Day7 {
    hands: Vec<Hand>
}

impl Day7 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day7 {
    fn name(&self) -> (usize, usize) {
        (2023, 7)
    }

    fn parse_input(&mut self) {
        let lines = utils::read_lines("./inputs/day7.txt");

        for line in &lines {
            let hand = line.split(" ").collect::<Vec<_>>()[0];
            let bid = line.split(" ").collect::<Vec<_>>()[1];
            println!("{:?} {:?}", hand, bid);

            let mut chars: Vec<char> = hand.chars().collect();
            chars.sort_by(|a, b| b.cmp(a));

            let mut distinct_cards = 0;
            // Hack cause cba with option
            let mut current_char: &char = &'_';
            let mut highest_num_chars = 0;
            for (i, char) in chars.iter().enumerate() {
                if current_char != char {
                    current_char = char;
                    distinct_cards += 1;
                    let temp = i - highest_num_chars;
                    if temp > highest_num_chars {
                        highest_num_chars = temp;
                    }
                } 
            }
            println!("{:?}", distinct_cards);

            if distinct_cards == 1 {
                let hand = Hand {
                    cards: chars,
                    bid: bid.parse::<i32>().unwrap(),
                    hand_type: HandType::FiveOfAKind
                };
                self.hands.push(hand);
            } else if distinct_cards == 2 {
                if highest_num_chars == 4 {
                    let hand = Hand {
                        cards: chars,
                        bid: bid.parse::<i32>().unwrap(),
                        hand_type: HandType::FourOfAKind
                    };
                    self.hands.push(hand);
                } else {
                    let hand = Hand {
                        cards: chars,
                        bid: bid.parse::<i32>().unwrap(),
                        hand_type: HandType::FullHouse
                    };
                    self.hands.push(hand);
                }
            } else if distinct_cards == 3 {
                if highest_num_chars == 3 {
                    let hand = Hand {
                        cards: chars,
                        bid: bid.parse::<i32>().unwrap(),
                        hand_type: HandType::ThreeOfAKind
                    };
                    self.hands.push(hand);
                } else {
                    let hand = Hand {
                        cards: chars,
                        bid: bid.parse::<i32>().unwrap(),
                        hand_type: HandType::TwoPair
                    };
                    self.hands.push(hand);
                }
            } else if distinct_cards == 4 {
                let hand = Hand {
                    cards: chars,
                    bid: bid.parse::<i32>().unwrap(),
                    hand_type: HandType::OnePair
                };
                self.hands.push(hand);
            } else if distinct_cards == 5 {
                let hand = Hand {
                    cards: chars,
                    bid: bid.parse::<i32>().unwrap(),
                    hand_type: HandType::HighCard
                };
                self.hands.push(hand);
            } else {
                panic!("We should not be here")
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        println!("{:?}", &self.hands);
        todo!()
    }

    fn part2(&mut self) -> Vec<String> {
        todo!()
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: i32,
}

enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
