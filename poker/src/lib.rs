/// Given a list of poker hands, return a list of those hands which win.
///
use core::panic;
use std::{array, cmp::Ordering};

use once_cell::sync::Lazy;
use std::collections::HashMap;
use thiserror::Error;

const SUIT_STRINGS: [&str; 4] = ["h", "d", "c", "s"];
const VALUE_STRINGS: [&str; 13] = [
    "a", "2", "3", "4", "5", "6", "7", "8", "9", "10", "j", "q", "k",
];

static SUIT_LOOKUP: Lazy<HashMap<&'static str, Suit>> = Lazy::new(|| {
    let mut m = HashMap::new();
    SUIT_STRINGS.iter().enumerate().for_each(|(i, &s)| {
        m.insert(s, Suit::values()[i]);
    });
    m
});

static VALUE_LOOKUP: Lazy<HashMap<&'static str, Value>> = Lazy::new(|| {
    let mut m = HashMap::new();
    VALUE_STRINGS.iter().enumerate().for_each(|(i, &s)| {
        m.insert(s, Value::values()[i]);
    });
    m
});

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    #[error("Bad value: {0}")]
    BadValue(String),

    #[error("Bad suit: {0}")]
    BadSuit(String),

    #[error("Bad card: {0}")]
    BadCard(String),

    #[error("Bad rank error")]
    BadRank,

    #[error("Bad hand error")]
    BadHand,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Heart,
    Diamond,
    Club,
    Spade,
}
impl Suit {
    pub fn values() -> [Self; 4] {
        [Suit::Heart, Suit::Diamond, Suit::Club, Suit::Spade]
    }
}

impl TryFrom<&str> for Suit {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SUIT_LOOKUP
            .get(value.to_lowercase().as_str())
            .cloned()
            .ok_or(Error::BadSuit(value.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rustfmt::skip]
pub enum Value {
    Ace = 14,
    Two = 2,
    Three,
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    pub fn value(self) -> u8 {
        self as u8
    }

    pub fn values() -> [Value; 13] {
        use Value::*;
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
    }
}

impl std::ops::Add<u8> for Value {
    type Output = u8;
    fn add(self, rhs: u8) -> Self::Output {
        self.value().add(rhs)
    }
}

impl std::ops::Add<Value> for u8 {
    type Output = u8;
    fn add(self, rhs: Value) -> Self::Output {
        self.add(rhs.value())
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Value::values()[value as usize]
    }
}

impl PartialEq<Value> for u8 {
    fn eq(&self, other: &Value) -> bool {
        *self == other.value()
    }
}

impl PartialEq<u8> for Value {
    fn eq(&self, other: &u8) -> bool {
        self.value() == *other
    }
}

impl TryFrom<&str> for Value {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        VALUE_LOOKUP
            .get(value.to_lowercase().as_str())
            .cloned()
            .ok_or(Error::BadValue(value.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card(Suit, Value);

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Self(suit, value)
    }

    pub fn suit(&self) -> Suit {
        self.0
    }

    pub fn value(&self) -> Value {
        self.1
    }
}

impl TryFrom<&str> for Card {
    type Error = Error;

    fn try_from(card: &str) -> Result<Self, Self::Error> {
        let len = card.len();
        if len != 2 && len != 3 {
            return Err(Error::BadCard("invalid length".to_string()));
        }
        let (v, s) = card.split_at(len - 1);
        Ok(Self(Suit::try_from(s)?, Value::try_from(v)?))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HoldemHand {
    cards: [Card; 5],
    rank: Rank,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Rank {
    HighCard([Value; 5]),
    Pair([Value; 4]),
    TwoPair([Value; 3]),
    Set([Value; 3]),
    Straight(Value),
    Flush([Value; 5]),
    FullHouse([Value; 2]),
    Bomb([Value; 2]),
    StraightFlush(Value),
    RoyalStraightFlush,
}

impl HoldemHand {
    fn new(mut cards: [Card; 5]) -> Self {
        cards.sort_by(|a, b| b.value().cmp(&a.value()));
        Self {
            cards,
            rank: Self::rank(&cards),
        }
    }

    fn rank(cards: &[Card; 5]) -> Rank {
        let mut counts = Vec::with_capacity(5);
        let mut is_flush = true;
        let mut is_straight = true;
        let mut pre = cards[0];
        counts.push((cards[0].value(), 1));
        let mut ind = 0;
        for cur in &cards[1..] {
            is_flush &= cur.suit() == pre.suit();
            is_straight &= cur.value() + 1 == pre.value()
                // "As 5c 4d 3h 2s" is straight
                || (pre.value() == Value::Ace && cur.value() == Value::Five);
            if cur.value() != pre.value() {
                counts.push((cur.value(), 1));
                ind += 1;
            } else {
                counts[ind].1 += 1;
            }
            pre = *cur;
        }
        counts.sort_by(|a, b| b.1.cmp(&a.1));
        match counts.len() {
            5 => {
                let val = array::from_fn(|i| counts[i].0);
                if is_straight {
                    if is_flush && cards[1].value() == Value::King {
                        return Rank::RoyalStraightFlush;
                    }
                    let v = if cards[0].value() == Value::Ace {
                        cards[1].value()
                    } else {
                        cards[0].value()
                    };
                    if is_flush {
                        return Rank::StraightFlush(v);
                    }
                    return Rank::Straight(v);
                }
                if is_flush {
                    return Rank::Flush(val);
                }
                return Rank::HighCard(val);
            }
            4 => return Rank::Pair(array::from_fn(|i| counts[i].0)),
            3 => {
                let val = array::from_fn(|i| counts[i].0);
                if counts[0].1 == 2 {
                    return Rank::TwoPair(val);
                }
                return Rank::Set(val);
            }
            2 => {
                let val = array::from_fn(|i| counts[i].0);
                if counts[0].1 == 3 {
                    return Rank::FullHouse(val);
                }
                return Rank::Bomb(val);
            }
            _ => panic!("no such rank invalid"),
        }
    }
}

impl TryFrom<&str> for HoldemHand {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = value
            .split_whitespace()
            .map(|s| Card::try_from(s))
            .collect::<Result<_, _>>()?;
        if cards.len() != 5 {
            return Err(Error::BadCard("invalid number of cards".to_string()));
        }
        Ok(Self::new(array::from_fn(|i| cards[i])))
    }
}

/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut pre = hands[0];
    let mut win = vec![pre];
    for cur in hands.iter().skip(1) {
        let first = HoldemHand::try_from(pre).unwrap();
        let second = HoldemHand::try_from(*cur).unwrap();
        println!(
            "{:?} {:?} {}",
            first.rank,
            second.rank,
            first.rank > second.rank
        );
        match first.rank.cmp(&second.rank) {
            Ordering::Less => {
                win.clear();
                win.push(*cur);
                pre = *cur
            }
            Ordering::Equal => {
                win.push(cur);
            }
            Ordering::Greater => win.push(pre),
        }
    }
    return win;
}
