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
        const OFFSET: usize = 2;
        let mut counts: [usize; 13] = [0; 13];
        
        for card in hand {
            counts[*card - OFFSET] += 1;
        }

        let mut two_count = 0;
        let mut three_count = 0;

        for curr in counts {
            if curr == 5 {
                return HandType::FiveOfAKind;
            }
            if curr == 4 {
                return HandType::FourOfAKind;
            }
            if curr == 2 {
                two_count += 1;
            }
            if curr == 3 {
                three_count += 1;
            }
        }

        let htype = if two_count == 1 && three_count == 1 {
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
        let hand = s.trim().chars().map(|c| match c {
            '2'..='9' => c.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!(),
        })
        .collect::<Vec<usize>>();

        // get hand type
        let htype = HandType::from(&hand);

        let result = Hand {
            htype,
            hand,
        };

        return Ok(result);
    }
}

fn main() {
    let mut hands_with_bids = INPUT.lines().map(|line| {
        let (hand_raw, bid_raw) = line.split_once(' ').unwrap();
        let hand = hand_raw.parse::<Hand>().unwrap();
        let bid = bid_raw.parse::<usize>().unwrap();

        return (hand, bid);
    })
    .collect::<Vec<_>>();

    hands_with_bids.sort_by(|x, y| x.0.cmp(&y.0));

    let result: usize = hands_with_bids
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum();

    dbg!(result);
}
