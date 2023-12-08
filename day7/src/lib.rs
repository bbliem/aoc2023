pub mod config;

use counter::Counter;
use core::panic;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Joker, // J in the second part
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Jack, // J in the first part
    Q,
    K,
    A,
}

impl Card {
    fn from_char(c: char, j_value: &Card) -> Result<Self, &'static str> {
        match c {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::T),
            'J' => Ok(j_value.to_owned()),
            'Q' => Ok(Self::Q),
            'K' => Ok(Self::K),
            'A' => Ok(Self::A),
            _ => Err("Invalid card type"),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type_for_cards(cards: &[Card; 5]) -> HandType {
    let mut card_counts = cards.iter().collect::<Counter<_>>();
    // Turn jokers into whatever else is most common
    let num_jokers = *card_counts.get(&Card::Joker).unwrap_or(&0);
    if num_jokers == 5 {
        return HandType::FiveOfAKind;
    }
    card_counts.remove(&Card::Joker);
    let most_common = card_counts.most_common();
    let mut most_common_iter = most_common.iter();
    let (_, count) = most_common_iter.next().unwrap();
    match count + num_jokers {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => if most_common_iter.next().unwrap().1 == 2 { HandType::FullHouse } else { HandType::ThreeOfAKind },
        2 => if most_common_iter.next().unwrap().1 == 2 { HandType::TwoPair } else { HandType::OnePair },
        1 => HandType::HighCard,
        _ => panic!("Unexpected card count")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    fn from_line(line: &str, j_value: &Card) -> Result<Self, Box<dyn Error>> {
        let mut line_iter = line.splitn(2, ' ');
        let cards = line_iter.next().ok_or("Could not read cards")?;
        let cards: Vec<Card> = cards.chars().map(|c| Card::from_char(c, j_value)).collect::<Result<_, _>>()?;
        if cards.len() != 5 {
            return Err("Hands must consist of five cards".into());
        }
        let cards: [Card; 5] = cards.try_into().unwrap();
        let bid = line_iter.next().ok_or("Could not read bid")?.parse()?;
        let hand_type = hand_type_for_cards(&cards);
        Ok(Hand { cards, bid, hand_type })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                self.cards.cmp(&other.cards)
            },
            type_ordering => type_ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct ScoredHand {
    hand: Hand,
    rank: i32,
    winnings: i32,
}

struct Puzzle {
    scored_hands: Vec<ScoredHand>,
}

impl Puzzle {
    fn from_input(input: &str, j_value: &Card) -> Result<Self, Box<dyn Error>> {
        let hands: Vec<Hand> = input.lines().map(|line| Hand::from_line(line, j_value)).collect::<Result<_, _>>()?;
        let mut sorted_hands: Vec<(usize, &Hand)> = hands.iter().enumerate().collect();
        sorted_hands.sort_by(|(_, a), (_, b)| a.cmp(b));
        let rank_of_hand_index = sorted_hands.into_iter().enumerate()
            .map(|(rank, (i, _))| (i, (rank + 1) as i32))
            .collect::<HashMap<_, _>>();
        // let mut hands_with_rank = vec![];
        // for (i, hand) in hands.drain(..).enumerate() {
        //     let rank = *rank_of_hand_index.get(&i).unwrap();
        //     hands_with_rank.push((hand, rank));
        // }
        // Or shorter:
        let hands_with_rank = hands.into_iter().enumerate()
            .map(|(i, hand)| (hand, *rank_of_hand_index.get(&i).unwrap()))
            .collect::<Vec<_>>();
        // let mut scored_hands = vec![];
        // for (hand, rank) in hands_with_rank {
        //     let winnings = rank * hand.bid;
        //     scored_hands.push(ScoredHand { hand, rank, winnings })
        // }
        // Or shorter:
        let scored_hands = hands_with_rank.into_iter()
            .map(|(hand, rank)| { let winnings = rank * hand.bid; ScoredHand { hand, rank, winnings } })
            .collect();
        Ok(Self { scored_hands })
    }
}

 fn part1(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, &Card::Jack)?;
    Ok(puzzle.scored_hands.iter().fold(0, |sum, scored_hand| sum + scored_hand.winnings))
}

fn part2(input: &str) -> Result<i32, Box<dyn Error>> {
    let puzzle = Puzzle::from_input(input, &Card::Joker)?;
    Ok(puzzle.scored_hands.iter().fold(0, |sum, scored_hand| sum + scored_hand.winnings))
}

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    println!("Part 1: Reading file {}", config.file_path1);
    let contents = fs::read_to_string(config.file_path1)?;
    let result = part1(&contents)?;
    println!("Result of part 1: {result}");

    println!("Part 2: Reading file {}", config.file_path2);
    let contents = fs::read_to_string(config.file_path2)?;
    let result = part2(&contents)?;
    println!("Result of part 2: {result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn a_stronger_than_two() {
        assert!(Card::A > Card::Two);
    }

    #[test]
    fn five_of_a_kind_stronger_than_four_of_a_kind() {
        let five = Hand::from_line("JJJJJ 0", &Card::Jack).unwrap();
        let four = Hand::from_line("JJJAJ 0", &Card::Jack).unwrap();
        assert!(five > four);
    }

    #[test]
    fn three_of_a_kind_weaker_than_full_house() {
        let three = Hand::from_line("44234 0", &Card::Jack).unwrap();
        let full_house = Hand::from_line("42424 0", &Card::Jack).unwrap();
        assert!(three < full_house);
    }

    #[test]
    fn tie_breaker1() {
        let stronger = Hand::from_line("33332 0", &Card::Jack).unwrap();
        let weaker = Hand::from_line("2AAAA 0", &Card::Jack).unwrap();
        assert!(stronger > weaker);
    }

    #[test]
    fn tie_breaker2() {
        let stronger = Hand::from_line("77888 0", &Card::Jack).unwrap();
        let weaker = Hand::from_line("77788 0", &Card::Jack).unwrap();
        assert!(stronger > weaker);
    }

    #[test]
    fn ranks() {
        let puzzle = Puzzle::from_input(EXAMPLE.trim(), &Card::Jack).unwrap();
        let expected_ranks = vec![1, 4, 3, 2, 5];
        for (scored_hand, rank) in puzzle.scored_hands.iter().zip(expected_ranks.iter()) {
            assert_eq!(scored_hand.rank, *rank);
        }
    }

    #[test]
    fn joker() {
        let four = Hand::from_line("QJJQ2 0", &Card::Joker).unwrap();
        assert_eq!(four.hand_type, HandType::FourOfAKind);
    }

    #[test]
    fn ranks_with_jokers() {
        let puzzle = Puzzle::from_input(EXAMPLE.trim(), &Card::Joker).unwrap();
        let expected_ranks = vec![1, 3, 2, 5, 4];
        for (scored_hand, rank) in puzzle.scored_hands.iter().zip(expected_ranks.iter()) {
            assert_eq!(scored_hand.rank, *rank);
        }
    }

    #[test]
    fn example_part1() -> Result<(), Box<dyn Error>> {
        let result = part1(EXAMPLE.trim())?;
        assert_eq!(result, 6440);
        Ok(())
    }

    #[test]
    fn example_part2() -> Result<(), Box<dyn Error>> {
        let result = part2(EXAMPLE.trim())?;
        assert_eq!(result, 5905);
        Ok(())
    }
}
