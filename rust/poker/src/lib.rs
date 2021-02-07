use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Card(u8, char);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum PokerHand {
    HighCard(u8, u8, u8, u8, u8),
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOfAKind(u8, u8, u8),
    Straight(u8),
    Flush(u8, u8, u8, u8, u8),
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush(u8),
}

impl Card {
    fn new(card_str: &str) -> Self {
        let suit = card_str.chars().last().unwrap();
        if card_str.len() == 3 {
            Card(10, suit)
        } else {
            let rank = match card_str.chars().next().unwrap() {
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                c => c.to_digit(10).unwrap(),
            };
            Card(rank as u8, suit)
        }
    }
}

impl PokerHand {
    fn new(cards: &mut [Card]) -> Self {
        cards.sort_by(|x, y| y.cmp(x));
        let straight = {
            cards.windows(2).all(|w| w[0].0 - w[1].0 == 1)
                || (cards.first().unwrap().0 == 14
                    && cards[1..].windows(2).all(|w| w[0].0 - w[1].0 == 1))
        };
        let flush = cards.iter().all(|Card(_, suit)| *suit == cards[0].1);
        let occurences = {
            let mut m = BTreeMap::new();
            cards.iter().for_each(|c| {
                let counter = m.entry(c.0).or_insert(0);
                *counter += 1;
            });
            let mut v = m.into_iter().collect::<Vec<_>>();
            v.sort_by(|x, y| match y.1.cmp(&x.1) {
                Ordering::Equal => y.0.cmp(&x.0),
                ord => ord,
            });
            v
        };

        if straight && flush {
            PokerHand::StraightFlush(cards[0].0)
        } else if occurences[0].1 == 4 {
            PokerHand::FourOfAKind(occurences[0].0, occurences[1].0)
        } else if occurences[0].1 == 3 && occurences[1].1 == 2 {
            PokerHand::FullHouse(occurences[0].0, occurences[1].0)
        } else if flush {
            PokerHand::Flush(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0)
        } else if straight {
            let x = {
                if cards[0].0 == 14 {
                    cards[1].0
                } else {
                    cards[0].0
                }
            };
            PokerHand::Straight(x)
        } else if occurences[0].1 == 3 {
            PokerHand::ThreeOfAKind(occurences[0].0, occurences[1].0, occurences[2].0)
        } else if occurences[0].1 == 2 && occurences[1].1 == 2 {
            PokerHand::TwoPair(occurences[0].0, occurences[1].0, occurences[2].0)
        } else if occurences[0].1 == 2 {
            PokerHand::OnePair(
                occurences[0].0,
                occurences[1].0,
                occurences[2].0,
                occurences[3].0,
            )
        } else {
            PokerHand::HighCard(cards[0].0, cards[1].0, cards[2].0, cards[3].0, cards[4].0)
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut poker_hands = hands
        .iter()
        .map(|s| {
            let mut cards = s.split(' ').map(Card::new).collect::<Vec<Card>>();
            (PokerHand::new(cards.as_mut_slice()), *s)
        })
        .collect::<Vec<(PokerHand, &'a str)>>();
    poker_hands.sort();
    let winner = poker_hands.iter().max().unwrap();
    let winners = poker_hands
        .iter()
        .filter_map(|x| if x.0 == winner.0 { Some(x.1) } else { None })
        .collect();
    Some(winners)
}
