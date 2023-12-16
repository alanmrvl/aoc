use std::str::FromStr;

const INPUT: &'static str = include_str!("../input");

// Deriving from PartialOrd will produce a lexicographic ordering based on the top-to-bottom order
// of the struct's members
// Source: https://doc.rust-lang.org/std/cmp/trait.Ord.html#derivable
// Source: https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#derivable
// Source: https://doc.rust-lang.org/std/cmp/trait.Ord.html#lexicographical-comparison
// We want to order first by the hand type and then by the lexicographical ordering of the hand
// itself
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    htype: HandType,
    hand: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from(hand: &Vec<usize>) -> Self {
        const OFFSET: usize = 1;
        let mut counts: [usize; 14] = [0; 14];

        for card in hand {
            counts[*card - OFFSET] += 1;
        }

        let mut two_count = 0;
        let mut three_count = 0;
        let mut four_count = 0;
        let mut five_count = 0;

        for curr in counts {
            if curr == 2 {
                two_count += 1;
            }
            if curr == 3 {
                three_count += 1;
            }
            if curr == 4 {
                four_count += 1;
            }
            if curr == 5 {
                five_count += 1;
            }
        }

        // calculate how J affects stuff
        let j_count = counts[0];

        // TODO: better way to do this part?
        match j_count {
            5 => (),
            4 => {
                five_count += 1;
                four_count -= 1;
            }
            3 => {
                if two_count == 1 {
                    five_count += 1;
                    two_count -= 1;
                } else {
                    four_count += 1;
                }
                three_count -= 1;
            }
            2 => {
                if three_count == 1 {
                    five_count += 1;
                    three_count -= 1;
                } else if two_count == 2 {
                    four_count += 1;
                    two_count -= 1;
                } else if two_count == 1 {
                    three_count += 1;
                }
                two_count -= 1;
            }
            1 => {
                if four_count == 1 {
                    five_count += 1;
                    four_count -= 1;
                } else if three_count == 1 {
                    four_count += 1;
                    three_count -= 1;
                } else if two_count > 0 {
                    three_count += 1;
                    two_count -= 1;
                } else {
                    two_count += 1;
                }
            }
            0 => (),
            _ => panic!(),
        }

        let htype = if five_count == 1 {
            HandType::FiveOfAKind
        } else if four_count == 1 {
            HandType::FourOfAKind
        } else if two_count == 1 && three_count == 1 {
            HandType::FullHouse
        } else if three_count == 1 {
            HandType::ThreeOfAKind
        } else if two_count == 2 {
            HandType::TwoPair
        } else if two_count == 1 {
            HandType::OnePair
        } else {
            HandType::HighCard
        };

        return htype;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand = s
            .trim()
            .chars()
            .map(|c| match c {
                'J' => 1,
                '2'..='9' => c.to_digit(10).unwrap() as usize,
                'T' => 10,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!(),
            })
            .collect::<Vec<usize>>();

        // get hand type
        let htype = HandType::from(&hand);

        let result = Hand { htype, hand };

        return Ok(result);
    }
}

fn main() {
    let mut hands_with_bids = INPUT
        .lines()
        .map(|line| {
            let (hand_raw, bid_raw) = line.split_once(' ').unwrap();
            let hand = hand_raw.parse::<Hand>().unwrap();
            let bid = bid_raw.parse::<usize>().unwrap();

            return (hand, bid);
        })
        .collect::<Vec<_>>();

    hands_with_bids.sort_by(|x, y| x.0.cmp(&y.0));

    // dbg!(&hands_with_bids);

    let result: usize = hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();

    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_jokers() {
        let vec: Vec<usize> = vec![1, 1, 1, 1, 1];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FiveOfAKind);
    }

    #[test]
    fn four_jokers() {
        let vec: Vec<usize> = vec![1, 1, 1, 1, 2];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FiveOfAKind);
    }

    #[test]
    fn three_jokers_to_five() {
        let vec: Vec<usize> = vec![1, 1, 1, 2, 2];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FiveOfAKind);
    }

    #[test]
    fn three_jokers_to_four() {
        let vec: Vec<usize> = vec![1, 1, 1, 2, 3];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FourOfAKind);
    }

    #[test]
    fn two_jokers_to_five() {
        let vec: Vec<usize> = vec![1, 1, 2, 2, 2];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FiveOfAKind);
    }

    #[test]
    fn two_jokers_to_four() {
        let vec: Vec<usize> = vec![1, 1, 2, 2, 3];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FourOfAKind);
    }

    // fullhouse is not possible because we can
    // always get four of a kind instead

    // #[test]
    // fn two_jokers_to_fullhouse() {
    //     let vec: Vec<usize> = vec![1, 1, ?, ?, ?];

    //     let res: HandType = HandType::from(&vec);

    //     assert_eq!(res, HandType::FullHouse);
    // }

    #[test]
    fn two_jokers_to_three() {
        let vec: Vec<usize> = vec![1, 1, 2, 3, 4];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::ThreeOfAKind);
    }

    #[test]
    fn one_joker_to_five() {
        let vec: Vec<usize> = vec![1, 2, 2, 2, 2];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FiveOfAKind);
    }

    #[test]
    fn one_joker_to_four() {
        let vec: Vec<usize> = vec![1, 2, 2, 2, 3];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FourOfAKind);
    }

    #[test]
    fn one_joker_to_fullhouse() {
        let vec: Vec<usize> = vec![1, 2, 2, 3, 3];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::FullHouse);
    }

    #[test]
    fn one_joker_to_three() {
        let vec: Vec<usize> = vec![1, 2, 2, 3, 4];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::ThreeOfAKind);
    }

    // twopair is not possible because we can
    // always get three of a kind instead

    // #[test]
    // fn one_joker_to_twopair() {
    //     let vec: Vec<usize> = vec![1, ?, ?, ?, ?];

    //     let res: HandType = HandType::from(&vec);

    //     assert_eq!(res, HandType::TwoPair);
    // }

    #[test]
    fn one_joker_to_two() {
        let vec: Vec<usize> = vec![1, 2, 3, 4, 5];

        let res: HandType = HandType::from(&vec);

        assert_eq!(res, HandType::OnePair);
    }

    // highcard is not possible because we can
    // always get pair instead

    // #[test]
    // fn one_joker_to_twopair() {
    //     let vec: Vec<usize> = vec![1, ?, ?, ?, ?];

    //     let res: HandType = HandType::from(&vec);

    //     assert_eq!(res, HandType::HighCard);
    // }
}
